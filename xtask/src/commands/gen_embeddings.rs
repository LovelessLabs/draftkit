//! Generate sqlite_vec embeddings for TailwindPlus components.
//!
//! This tool reads NDJSON component files and generates semantic embeddings
//! using the all-MiniLM-L6-v2 model via candle. Components are deduplicated
//! by ID and the database tracks which frameworks/versions are available.
#![allow(unsafe_code)] // Required for sqlite_vec FFI

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use clap::Parser;
use hf_hub::{api::tokio::Api, Repo, RepoType};
use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use serde::Deserialize;
use sqlite_vec::sqlite3_vec_init;
use tokenizers::Tokenizer;
use tracing::{debug, info};

const MODEL_ID: &str = "sentence-transformers/all-MiniLM-L6-v2";

#[derive(Parser, Debug)]
#[command(about = "Generate sqlite_vec embeddings for TailwindPlus components")]
pub struct GenEmbeddingsArgs {
    /// Directory containing NDJSON files (e.g., cache/2026-01-12/data/components/)
    #[arg(long, short = 'i')]
    input_dir: PathBuf,

    /// Output path for embeddings.db
    #[arg(long, short = 'o', default_value = "data/embeddings.db")]
    output: PathBuf,

    /// Batch size for embedding generation
    #[arg(long, default_value = "32")]
    batch_size: usize,
}

/// Component record from NDJSON file (one per line).
#[derive(Debug, Deserialize)]
struct NdjsonComponent {
    id: String,
    name: String,
    category: String,
    subcategory: String,
    sub_subcategory: String,
    version: String, // "v3" or "v4"
    // light/dark/system fields ignored for embeddings
}

/// Deduplicated component metadata for embedding.
#[derive(Debug)]
struct ComponentMeta {
    id: String,
    name: String,
    category: String,
    subcategory: String,
    sub_subcategory: String,
    versions: HashSet<String>,
    frameworks: HashSet<String>,
}

/// Mean pooling over the token embeddings.
fn mean_pooling(embeddings: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
    let mask = attention_mask.unsqueeze(2)?.to_dtype(embeddings.dtype())?;
    let masked = embeddings.broadcast_mul(&mask)?;
    let sum = masked.sum(1)?;
    let count = mask.sum(1)?.clamp(1e-9, f64::MAX)?;
    Ok(sum.broadcast_div(&count)?)
}

/// L2 normalize the embeddings.
fn normalize(embeddings: &Tensor) -> Result<Tensor> {
    let norm = embeddings.sqr()?.sum_keepdim(1)?.sqrt()?;
    Ok(embeddings.broadcast_div(&norm)?)
}

/// Extract framework from filename (e.g., "react-v4.ndjson" -> "react").
fn framework_from_filename(filename: &str) -> Option<&str> {
    filename.strip_suffix(".ndjson").and_then(|s| s.split('-').next())
}

/// Read all NDJSON files in directory and deduplicate by component ID.
fn load_components(input_dir: &PathBuf) -> Result<Vec<ComponentMeta>> {
    let mut components: HashMap<String, ComponentMeta> = HashMap::new();

    let entries: Vec<_> = std::fs::read_dir(input_dir)
        .context("Failed to read input directory")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "ndjson")
        })
        .collect();

    info!("Found {} NDJSON files", entries.len());

    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        let framework = framework_from_filename(&filename).unwrap_or("unknown");

        debug!("Reading {}", path.display());

        let file = File::open(&path).with_context(|| format!("Failed to open {}", path.display()))?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.with_context(|| format!("Failed to read line {} in {}", line_num + 1, path.display()))?;
            if line.trim().is_empty() {
                continue;
            }

            let component: NdjsonComponent = serde_json::from_str(&line)
                .with_context(|| format!("Failed to parse line {} in {}", line_num + 1, path.display()))?;

            components
                .entry(component.id.clone())
                .and_modify(|c| {
                    c.versions.insert(component.version.clone());
                    c.frameworks.insert(framework.to_string());
                })
                .or_insert_with(|| ComponentMeta {
                    id: component.id,
                    name: component.name,
                    category: component.category,
                    subcategory: component.subcategory,
                    sub_subcategory: component.sub_subcategory,
                    versions: HashSet::from([component.version]),
                    frameworks: HashSet::from([framework.to_string()]),
                });
        }
    }

    let mut result: Vec<_> = components.into_values().collect();
    result.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(result)
}

pub async fn cmd_gen_embeddings(args: GenEmbeddingsArgs) -> Result<()> {
    info!("Loading components from {:?}", args.input_dir);
    let components = load_components(&args.input_dir)?;
    info!("Found {} unique components", components.len());

    if components.is_empty() {
        anyhow::bail!("No components found in input directory");
    }

    // Download model from HuggingFace Hub
    info!("Loading model: {}", MODEL_ID);
    let api = Api::new()?;
    let repo = api.repo(Repo::new(MODEL_ID.to_string(), RepoType::Model));

    let config_path = repo.get("config.json").await?;
    let tokenizer_path = repo.get("tokenizer.json").await?;
    let weights_path = repo.get("model.safetensors").await?;

    // Load config and tokenizer
    let config: Config = serde_json::from_reader(std::fs::File::open(&config_path)?)?;
    let tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(|e| anyhow::anyhow!("{e}"))?;

    // Load model weights
    let device = Device::Cpu; // Use CPU for simplicity; Metal available via accelerate feature
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DTYPE, &device)? };
    let model = BertModel::load(vb, &config)?;

    info!("Model loaded successfully");

    // Ensure output directory exists
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Load sqlite_vec extension before opening any connection
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(
                *mut rusqlite::ffi::sqlite3,
                *mut *mut i8,
                *const rusqlite::ffi::sqlite3_api_routines,
            ) -> i32,
        >(sqlite3_vec_init as *const ())));
    }

    // Create output database
    info!("Creating embeddings database at {:?}", args.output);
    let mut conn = Connection::open(&args.output)?;

    // Create tables
    conn.execute_batch(
        r#"
        DROP TABLE IF EXISTS component_embeddings;
        DROP TABLE IF EXISTS components;

        CREATE TABLE components (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            subcategory TEXT NOT NULL,
            sub_subcategory TEXT NOT NULL,
            versions TEXT NOT NULL,
            frameworks TEXT NOT NULL
        );

        CREATE VIRTUAL TABLE component_embeddings USING vec0(
            id TEXT PRIMARY KEY,
            embedding FLOAT[384]
        );
        "#,
    )?;

    // Process components in batches
    let progress = ProgressBar::new(components.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"),
    );

    let tx = conn.transaction()?;

    for batch in components.chunks(args.batch_size) {
        // Create search text for each component
        // Format: "category subcategory sub_subcategory name"
        let texts: Vec<String> = batch
            .iter()
            .map(|c| {
                format!(
                    "{} {} {} {}",
                    c.category, c.subcategory, c.sub_subcategory, c.name
                )
            })
            .collect();

        // Tokenize
        let encodings = tokenizer
            .encode_batch(texts.clone(), true)
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        // Find max length for padding
        let max_len = encodings.iter().map(|e| e.get_ids().len()).max().unwrap_or(0);

        // Create input tensors
        let mut input_ids_vec = Vec::new();
        let mut attention_mask_vec = Vec::new();

        for encoding in &encodings {
            let ids = encoding.get_ids();
            let mask = encoding.get_attention_mask();

            // Pad to max_len
            let mut padded_ids = ids.to_vec();
            let mut padded_mask = mask.to_vec();
            padded_ids.resize(max_len, 0);
            padded_mask.resize(max_len, 0);

            input_ids_vec.extend(padded_ids.iter().map(|&x| x as i64));
            attention_mask_vec.extend(padded_mask.iter().map(|&x| x as i64));
        }

        let batch_size = encodings.len();
        let input_ids =
            Tensor::from_vec(input_ids_vec, (batch_size, max_len), &device)?.to_dtype(DType::U32)?;
        let attention_mask = Tensor::from_vec(attention_mask_vec, (batch_size, max_len), &device)?;
        let token_type_ids = input_ids.zeros_like()?;

        // Run model
        let embeddings = model.forward(&input_ids, &token_type_ids, Some(&attention_mask))?;

        // Mean pooling and normalization
        let pooled = mean_pooling(&embeddings, &attention_mask)?;
        let normalized = normalize(&pooled)?;

        // Insert into database
        for (i, component) in batch.iter().enumerate() {
            // Format versions and frameworks as comma-separated strings
            let mut versions: Vec<_> = component.versions.iter().cloned().collect();
            versions.sort();
            let versions_str = versions.join(",");

            let mut frameworks: Vec<_> = component.frameworks.iter().cloned().collect();
            frameworks.sort();
            let frameworks_str = frameworks.join(",");

            // Insert component metadata
            tx.execute(
                "INSERT INTO components (id, name, category, subcategory, sub_subcategory, versions, frameworks) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    component.id,
                    component.name,
                    component.category,
                    component.subcategory,
                    component.sub_subcategory,
                    versions_str,
                    frameworks_str,
                ],
            )?;

            // Extract embedding for this component
            let embedding: Vec<f32> = normalized.get(i)?.to_vec1()?;
            let embedding_bytes: Vec<u8> = embedding.iter().flat_map(|f| f.to_le_bytes()).collect();

            // Insert embedding
            tx.execute(
                "INSERT INTO component_embeddings (id, embedding) VALUES (?1, ?2)",
                rusqlite::params![component.id, embedding_bytes],
            )?;

            progress.inc(1);
        }
    }

    tx.commit()?;
    progress.finish_with_message("Done!");

    // Verify
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM components", [], |row| row.get(0))?;
    info!("Created embeddings for {} components", count);

    // Show sample search
    info!("Testing search for 'sidebar dark dropdown'...");

    // Generate query embedding
    let query = "sidebar dark dropdown";
    let encoding = tokenizer
        .encode(query, true)
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let input_ids = Tensor::from_vec(
        encoding
            .get_ids()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>(),
        (1, encoding.get_ids().len()),
        &device,
    )?
    .to_dtype(DType::U32)?;
    let attention_mask = Tensor::from_vec(
        encoding
            .get_attention_mask()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>(),
        (1, encoding.get_ids().len()),
        &device,
    )?;
    let token_type_ids = input_ids.zeros_like()?;

    let embeddings = model.forward(&input_ids, &token_type_ids, Some(&attention_mask))?;
    let pooled = mean_pooling(&embeddings, &attention_mask)?;
    let normalized = normalize(&pooled)?;
    let query_embedding: Vec<f32> = normalized.get(0)?.to_vec1()?;
    let query_bytes: Vec<u8> = query_embedding
        .iter()
        .flat_map(|f| f.to_le_bytes())
        .collect();

    // Search
    let mut stmt = conn.prepare(
        r#"
        SELECT c.id, c.name, c.category, c.subcategory, c.frameworks, vec_distance_cosine(e.embedding, ?1) as distance
        FROM components c
        JOIN component_embeddings e ON c.id = e.id
        ORDER BY distance ASC
        LIMIT 5
        "#,
    )?;

    let results: Vec<(String, String, String, String, String, f64)> = stmt
        .query_map([query_bytes], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    for (_id, name, category, subcategory, frameworks, distance) in results {
        info!(
            "  {:.4} - {} / {} / {} [{}]",
            1.0 - distance, // Convert distance to similarity
            category,
            subcategory,
            name,
            frameworks
        );
    }

    Ok(())
}

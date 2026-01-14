//! Observability setup: JSONL file logs + optional OpenTelemetry tracing.

use anyhow::Result;
use opentelemetry::KeyValue;
use opentelemetry::trace::{TraceContextExt, TracerProvider as _};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::resource::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use serde_json::{Map, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{Event, Span, field};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::time::{FormatTime, UtcTime};
use tracing_subscriber::layer::{Context, SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;

const ENV_APP_ENV: &str = "APP_ENV";
const ENV_LOG_PATH: &str = "APP_LOG_PATH";
const ENV_LOG_DIR: &str = "APP_LOG_DIR";
const ENV_OTEL_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";
const DEFAULT_LOG_DIR_UNIX: &str = "/var/log";
const LOG_FILE_SUFFIX: &str = ".jsonl";

#[derive(Clone, Debug)]
pub struct ObservabilityConfig {
    pub service: String,
    pub env: String,
    pub version: String,
    pub otlp_endpoint: Option<String>,
    pub log_dir: Option<PathBuf>,
}

impl ObservabilityConfig {
    pub fn from_env_with_overrides(
        otlp_endpoint: Option<String>,
        log_dir: Option<PathBuf>,
    ) -> Self {
        let env_endpoint = std::env::var(ENV_OTEL_ENDPOINT)
            .ok()
            .filter(|value| !value.trim().is_empty());

        Self {
            service: env!("CARGO_PKG_NAME").to_string(),
            env: std::env::var(ENV_APP_ENV).unwrap_or_else(|_| "dev".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
            otlp_endpoint: env_endpoint.or(otlp_endpoint),
            log_dir,
        }
    }
}

#[derive(Clone, Debug)]
struct LogTarget {
    dir: PathBuf,
    file_name: String,
}

impl LogTarget {
    #[cfg(test)]
    fn path(&self) -> PathBuf {
        self.dir.join(&self.file_name)
    }
}

pub struct ObservabilityGuard {
    tracer_provider: Option<SdkTracerProvider>,
    _log_guard: tracing_appender::non_blocking::WorkerGuard,
}

impl Drop for ObservabilityGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.tracer_provider.take() {
            if let Err(err) = provider.shutdown() {
                eprintln!("Error shutting down tracer provider: {err}");
            }
        }
    }
}

pub fn init_observability(
    cfg: &ObservabilityConfig,
    env_filter: EnvFilter,
) -> Result<ObservabilityGuard> {
    let (log_writer, log_guard) = match build_log_writer(&cfg.service, cfg.log_dir.as_deref()) {
        Ok(result) => result,
        Err(err) => {
            // Use stderr for fallback logging to avoid corrupting MCP/stdio protocols
            eprintln!("Warning: {err}. Falling back to stderr logging.");
            let (writer, guard) = tracing_appender::non_blocking(std::io::stderr());
            (writer, guard)
        }
    };

    let log_layer = JsonLogLayer::new(log_writer);

    let (otel_layer, tracer_provider) = if let Some(endpoint) = cfg.otlp_endpoint.as_ref() {
        let resource = Resource::builder()
            .with_attributes([
                KeyValue::new("service.name", cfg.service.clone()),
                KeyValue::new("deployment.environment", cfg.env.clone()),
                KeyValue::new("service.version", cfg.version.clone()),
            ])
            .build();

        let exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint.clone())
            .build()?;

        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(resource)
            .build();

        let tracer = tracer_provider.tracer("draftkit");
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        (Some(otel_layer), Some(tracer_provider))
    } else {
        (None, None)
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(otel_layer)
        .with(log_layer)
        .init();

    Ok(ObservabilityGuard {
        tracer_provider,
        _log_guard: log_guard,
    })
}

/// Create a span that carries Datadog-friendly tags and OTel correlation IDs.
pub fn correlated_span(name: &'static str, cfg: &ObservabilityConfig) -> Span {
    // Note: tracing macros require literal span names, but we use the name
    // parameter for semantic clarity at call sites. The actual span is "cli".
    let _ = name;
    tracing::info_span!(
        "cli",
        service = field::display(&cfg.service),
        env = field::display(&cfg.env),
        version = field::display(&cfg.version),
        trace_id = field::Empty,
        span_id = field::Empty,
    )
}

/// Record OpenTelemetry trace/span IDs into the provided span.
pub fn record_otel_ids(span: &Span) {
    let span_ctx = span.context().span().span_context().clone();
    if span_ctx.is_valid() {
        span.record("trace_id", field::display(span_ctx.trace_id().to_string()));
        span.record("span_id", field::display(span_ctx.span_id().to_string()));
    }
}

/// Deprecated: Shutdown is now handled via [`ObservabilityGuard::drop`].
/// This function is kept for API compatibility but does nothing.
pub const fn shutdown_tracing() {
    // No-op: shutdown is handled when ObservabilityGuard is dropped
}

pub fn env_filter(quiet: bool, verbose: u8, default_level: &str) -> EnvFilter {
    if quiet {
        return EnvFilter::new("error");
    }

    if verbose > 0 {
        let level = match verbose {
            1 => "debug",
            _ => "trace",
        };
        return EnvFilter::new(level);
    }

    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level))
}

struct JsonLogLayer<W> {
    writer: W,
}

impl<W> JsonLogLayer<W> {
    const fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<S, W> tracing_subscriber::Layer<S> for JsonLogLayer<W>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    W: for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + Send + Sync + 'static,
{
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: Context<'_, S>,
    ) {
        if let Some(span) = ctx.span(id) {
            let mut visitor = JsonVisitor::default();
            attrs.record(&mut visitor);
            span.extensions_mut().insert(SpanFields {
                values: visitor.values,
            });
        }
    }

    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: Context<'_, S>,
    ) {
        if let Some(span) = ctx.span(id) {
            let mut visitor = JsonVisitor::default();
            values.record(&mut visitor);
            let mut extensions = span.extensions_mut();
            if let Some(fields) = extensions.get_mut::<SpanFields>() {
                fields.values.extend(visitor.values);
            } else {
                extensions.insert(SpanFields {
                    values: visitor.values,
                });
            }
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut map = Map::new();

        let timestamp = format_timestamp();
        map.insert("timestamp".to_string(), Value::String(timestamp));
        map.insert(
            "level".to_string(),
            Value::String(event.metadata().level().as_str().to_lowercase()),
        );
        map.insert(
            "target".to_string(),
            Value::String(event.metadata().target().to_string()),
        );

        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                if let Some(fields) = span.extensions().get::<SpanFields>() {
                    map.extend(fields.values.clone());
                }
            }
        }

        let mut visitor = JsonVisitor::default();
        event.record(&mut visitor);
        map.extend(visitor.values);

        let mut writer = self.writer.make_writer();
        if serde_json::to_writer(&mut writer, &Value::Object(map)).is_ok() {
            let _ = writer.write_all(b"\n");
        }
    }
}

#[derive(Clone, Debug)]
struct SpanFields {
    values: Map<String, Value>,
}

#[derive(Default)]
struct JsonVisitor {
    values: Map<String, Value>,
}

impl tracing::field::Visit for JsonVisitor {
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.values
            .insert(field.name().to_string(), Value::Bool(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.values
            .insert(field.name().to_string(), Value::Number(value.into()));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.values
            .insert(field.name().to_string(), Value::Number(value.into()));
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        if let Some(number) = serde_json::Number::from_f64(value) {
            self.values
                .insert(field.name().to_string(), Value::Number(number));
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.values
            .insert(field.name().to_string(), Value::String(value.to_string()));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.values
            .insert(field.name().to_string(), Value::String(value.to_string()));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.values.insert(
            field.name().to_string(),
            Value::String(format!("{value:?}")),
        );
    }
}

fn format_timestamp() -> String {
    use tracing_subscriber::fmt::format::Writer;

    let timer = UtcTime::rfc_3339();
    let mut buffer = String::new();
    // FormatTime::format_time expects a Writer<'_>, which wraps fmt::Write
    let mut writer = Writer::new(&mut buffer);
    let _ = timer.format_time(&mut writer);
    buffer
}

fn build_log_writer(
    service: &str,
    config_log_dir: Option<&Path>,
) -> Result<(
    tracing_appender::non_blocking::NonBlocking,
    tracing_appender::non_blocking::WorkerGuard,
)> {
    let target = resolve_log_target(service, config_log_dir).map_err(anyhow::Error::msg)?;

    let appender = tracing_appender::rolling::daily(&target.dir, &target.file_name);
    let (writer, guard) = tracing_appender::non_blocking(appender);

    Ok((writer, guard))
}

fn resolve_log_target(service: &str, config_log_dir: Option<&Path>) -> Result<LogTarget, String> {
    let path_override = std::env::var_os(ENV_LOG_PATH).map(PathBuf::from);
    let dir_override = std::env::var_os(ENV_LOG_DIR).map(PathBuf::from);

    resolve_log_target_with(
        service,
        path_override,
        dir_override,
        config_log_dir.map(PathBuf::from),
    )
}

fn resolve_log_target_with(
    service: &str,
    path_override: Option<PathBuf>,
    dir_override: Option<PathBuf>,
    config_dir: Option<PathBuf>,
) -> Result<LogTarget, String> {
    if let Some(path) = path_override {
        return log_target_from_path(path);
    }

    if let Some(dir) = dir_override {
        return log_target_from_dir(dir, service);
    }

    if let Some(dir) = config_dir {
        return log_target_from_dir(dir, service);
    }

    let mut candidates = Vec::new();

    if cfg!(unix) {
        candidates.push(PathBuf::from(DEFAULT_LOG_DIR_UNIX));
    }

    if let Some(dir) = dirs::data_local_dir() {
        candidates.push(dir.join(service).join("logs"));
    }

    if let Ok(dir) = std::env::current_dir() {
        candidates.push(dir);
    }

    let file_name = format!("{service}{LOG_FILE_SUFFIX}");

    for dir in candidates {
        if ensure_writable(&dir, &file_name).is_ok() {
            return Ok(LogTarget { dir, file_name });
        }
    }

    Err("No writable log directory found".to_string())
}

fn log_target_from_dir(dir: PathBuf, service: &str) -> Result<LogTarget, String> {
    let file_name = format!("{service}{LOG_FILE_SUFFIX}");
    ensure_writable(&dir, &file_name)?;
    Ok(LogTarget { dir, file_name })
}

fn log_target_from_path(path: PathBuf) -> Result<LogTarget, String> {
    let file_name = path
        .file_name()
        .ok_or_else(|| "APP_LOG_PATH must include a file name".to_string())
        .and_then(|name| {
            name.to_str()
                .map(|value| value.to_string())
                .ok_or_else(|| "APP_LOG_PATH must be valid UTF-8".to_string())
        })?;

    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    ensure_writable(dir, &file_name)?;

    Ok(LogTarget {
        dir: dir.to_path_buf(),
        file_name,
    })
}

fn ensure_writable(dir: &Path, file_name: &str) -> Result<(), String> {
    std::fs::create_dir_all(dir)
        .map_err(|e| format!("Failed to create log directory {}: {e}", dir.display()))?;

    let path = dir.join(file_name);
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("Failed to open log file {}: {e}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        LOG_FILE_SUFFIX, env_filter, log_target_from_dir, log_target_from_path,
        resolve_log_target_with,
    };

    #[test]
    fn env_filter_quiet_overrides() {
        let filter = env_filter(true, 0, "info");
        assert_eq!(filter.to_string(), "error");
    }

    #[test]
    fn env_filter_verbose_maps_to_debug_and_trace() {
        let debug_filter = env_filter(false, 1, "info");
        assert_eq!(debug_filter.to_string(), "debug");

        let trace_filter = env_filter(false, 2, "info");
        assert_eq!(trace_filter.to_string(), "trace");
    }

    #[test]
    fn log_target_from_path_uses_parent_dir() {
        let temp_dir = std::env::temp_dir().join("draftkit-log-path");
        let file_path = temp_dir.join("custom.jsonl");

        let target = log_target_from_path(file_path).expect("log target from path");
        assert_eq!(target.dir, temp_dir);
        assert_eq!(target.file_name, "custom.jsonl");
    }

    #[test]
    fn log_target_from_dir_appends_file_name() {
        let temp_dir = std::env::temp_dir().join("draftkit-log-dir");
        let target = log_target_from_dir(temp_dir.clone(), "demo").expect("log target from dir");
        assert_eq!(target.dir, temp_dir);
        assert_eq!(target.file_name, format!("demo{LOG_FILE_SUFFIX}"));
    }

    #[test]
    fn resolve_log_target_with_prefers_path_override() {
        let temp_dir = std::env::temp_dir().join("draftkit-log-override");
        let file_path = temp_dir.join("override.jsonl");

        let target = resolve_log_target_with("demo", Some(file_path.clone()), None, None)
            .expect("override log target");

        assert_eq!(target.path(), file_path);
    }

    #[test]
    fn resolve_log_target_with_falls_back_to_dir_override() {
        let temp_dir = std::env::temp_dir().join("draftkit-log-dir-override");
        let target = resolve_log_target_with("demo", None, Some(temp_dir.clone()), None)
            .expect("dir override log target");

        assert_eq!(target.dir, temp_dir);
        assert_eq!(target.file_name, format!("demo{LOG_FILE_SUFFIX}"));
    }

    #[test]
    fn resolve_log_target_with_uses_config_dir() {
        let temp_dir = std::env::temp_dir().join("draftkit-log-config-dir");
        let target = resolve_log_target_with("demo", None, None, Some(temp_dir.clone()))
            .expect("config dir log target");

        assert_eq!(target.dir, temp_dir);
        assert_eq!(target.file_name, format!("demo{LOG_FILE_SUFFIX}"));
    }

    #[test]
    fn resolve_log_target_with_uses_temp_dir_when_forced() {
        let temp_dir = std::env::temp_dir().join("draftkit-log-fallback");
        let target = resolve_log_target_with("demo", None, Some(temp_dir.clone()), None)
            .expect("fallback log target");

        assert_eq!(target.dir, temp_dir);
    }
}

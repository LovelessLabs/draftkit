# Repository Guidelines

## Project Structure
- Consider the project a monorepo containing multiple crates unless explicitly told otherwise.
- Follow the existing crate/workspace layout. If creating a new workspace, prefer a simple, discoverable structure (e.g. crates under `crates/`).
- Treat `target/` as build output only; do not commit artifacts.
- Capture architectural decisions in Markdown ADRs under `docs/decisions/`.

## Dependencies
- NEVER add directly to `Cargo.toml` files.
- ALWAYS use `cargo add`
- For the following types of functionality, use:
  - CLI: `clap` (derive) + `clap_complete` for completions, `clap_mangen` for manpages
  - Errors: `thiserror` in libraries; `anyhow` in binaries.
  - Logging: `tracing` + `tracing_subscriber`.
  - YAML: `saphyr`
  - Serialization: `serde` + `serde_json` + `serde-saphyr`
  - Testing (CLI): `assert_cmd` + `predicates`; snapshots via `insta`.
  - Async: `tokio` when async is required; otherwise stay sync.
  - HTTP: `reqwest` (client), `axum` (server) when needed.
  - Paths: `camino`, since we don't always know when UTF-8 paths are required.
  - Testing: `cargo nextest`, `cargo llvm-cov nextest` for coverage.
  - Configuration: `config`

## Build, Test, and Development Commands
- `just --list` — show available recipes (preferred entrypoint if `just` is installed).
- `just check` — format, lint, deny, tests (nextest), and doctests.
- `just fmt` — format.
- `just clippy` — lint (CI-level strict).
- `just deny` — check dependencies for security advisories and license compliance (requires `cargo-deny`).
- `just test` (or `cargo nt`) — tests (requires `cargo-nextest`).
- `just doc-test` (or `cargo doc-test`) — doctests.
- `cargo check --all-targets --all-features` — fast compile sanity check.
- Optional: `just cov` (or `cargo cov`) — coverage (requires `cargo-llvm-cov`).
- Optional: `markdownlint` — lint Markdown files.

## Coding Style & Naming Conventions
- Rust 2024 edition; `#![deny(unsafe_code)]` in each crate — keep contributions safe and pure Rust.
- Prefer idiomatic, readable Rust; rely on `rustfmt` defaults.
- Keep clippy clean; avoid `unsafe` unless there’s a clear, documented need.
- Use standard Rust naming: `snake_case` (items), `UpperCamelCase` (types/traits).
- For public APIs: add rustdoc, examples when helpful, and keep semver impact in mind.

## Testing Guidelines
- If you change behavior, add/adjust tests in the same change.
- Prefer fast, deterministic tests; use integration tests for end-to-end behavior.
- `just cov` output should exceed 90% prior to any commit.

## Commit & Pull Request Guidelines
- Follow Conventional Commits (`feat:`, `fix:`, `docs:`, `perf:`, `chore:`) for clarity.
- PRs ALSO follow conventional commit style, and should include: purpose, scope of files touched, manual checks performed, and open questions.
- Link related issues if available.
- Never make commits directly. Write commit message to `commit.txt`. If branch is clean, make sure `commit.txt` is empty before writing to it. Otherwise, append since changes are accumulating prior to commit.

## MSRV
- Set `package.rust-version` in `Cargo.toml` (if not already set) and keep it intentional.
- CI runs an MSRV compile check when `rust-version` is present.

## Docs
- If you capture architectural decisions, prefer lightweight ADRs under `docs/decisions/`.

## Releases
- Publishing to crates.io requires the `CARGO_REGISTRY_TOKEN` GitHub Actions secret.
- Keep `CHANGELOG.md` up to date for user-visible changes.

## Security & Configuration
- Do not commit secrets. Prefer environment variables for credentials and document configuration defaults.
- `deny.toml` configures `cargo-deny` for supply chain security: license compliance, vulnerability scanning, and source verification.

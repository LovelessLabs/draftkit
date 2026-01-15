set shell := ["zsh", "-c"]
set dotenv-load := true

default:
  @just --list

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

# Check dependencies for security advisories and license compliance
deny:
  cargo deny check

test:
  cargo nextest run

test-ci:
  cargo nextest run --profile ci

doc-test:
  cargo test --doc

cov:
  @cargo llvm-cov clean --workspace
  cargo llvm-cov nextest --no-report
  @cargo llvm-cov report --html
  @cargo llvm-cov report --summary-only --json --output-path target/llvm-cov/summary.json

check: fmt clippy deny test doc-test

# Run all benchmarks
bench:
  cargo xtask bench

# Run divan (wall-clock) benchmarks only
bench-divan:
  cargo bench --bench divan_benchmarks

# Run gungraun (instruction count) benchmarks only
bench-gungraun:
  cargo bench --bench gungraun_benchmarks

# Run CLI (hyperfine) benchmarks only
bench-cli:
  ./scripts/bench-cli.sh

# Collect TailwindPlus data
# Examples:
#   just collect              # v3 & v4 → cache/YYYY-MM-DD/
#   just collect foo          # with suffix → cache/YYYY-MM-DD-foo/
#   just collect --v4-only    # don't include v3 formats
#   just collect --resume     # resume after failure
#   just collect --v4-only --resume foo
collect *ARGS:
  ./scripts/tailwindplus-collector.sh {{ARGS}}

# Clean TailwindPlus cached data
clean-cache:
  rm -Rf cache/*

# Generate embeddings from collected NDJSON data
# Examples:
#   just gen-embeddings cache/2026-01-12/data/components
#   just gen-embeddings cache/latest/data/components -o custom.db
gen-embeddings INPUT_DIR *ARGS:
  cargo xtask gen-embeddings --input-dir {{INPUT_DIR}} {{ARGS}}

# Run MCP server (stdio transport, data embedded at compile time)
serve:
  cargo run -p draftkit --release -- serve

# Build release binary
build-release:
  cargo build -p draftkit --release

zip:
  git archive --format=zip --output=../draftkit-{{datetime('-%Y-%m-%d_%H%M')}}.zip HEAD

# Check for outdated dependencies (root only, no transitive noise)
outdated:
    cargo outdated --workspace --root-deps-only

# Safe update: respects semver constraints, only touches Cargo.lock
update:
    cargo update --workspace --verbose

# Upgrade Cargo.toml to latest compatible versions
upgrade:
    cargo upgrade --workspace
    cargo update --workspace

# The nuclear option: upgrade to latest incompatible versions (breaking changes)
upgrade-breaking:
    cargo upgrade --workspace --incompatible
    cargo update --workspace

# See what WOULD update without doing it
check-updates:
    cargo update --workspace --dry-run

# Full refresh: update, test, clippy
refresh: update
    cargo test --workspace
    cargo clippy --workspace -- -D warnings

# Monthly maintenance: upgrade, test everything
monthly: upgrade
    cargo test --workspace
    cargo clippy --workspace -- -D warnings
    cargo build --workspace --release

# Show why a specific package version is stuck
[private]
why pkg:
    cargo tree -i {{pkg}}

# Update system-level cargo tools
system-update:
    cargo install-update -al

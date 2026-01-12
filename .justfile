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

zip:
  git archive --format=zip --output=../draftkit-{{datetime('-%Y-%m-%d_%H%M')}}.zip HEAD

# Check for dependency updates across the workspace (requires cargo-outdated)
deps:
    cargo outdated --workspace --root-deps-only

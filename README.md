# draftkit

MCP server for Tailwind Plus members.

## Features

- **Hierarchical Configuration** - Automatic config discovery from project directories up to user home
- **Structured Logging** - Daily-rotated JSONL log files with optional OTel correlation
- **JSON Output** - Machine-readable output for scripting and automation
- **Shell Completions** - Tab completion for Bash, Zsh, Fish, and PowerShell
- **Man Pages** - Unix-style documentation

## Installation

### Homebrew (macOS and Linux)

```bash
brew install lovelesslabs/brew/draftkit
```

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/lovelesslabs/draftkit/releases).

Binaries are available for:
- macOS (Apple Silicon and Intel)
- Linux (x86_64 and ARM64, glibc and musl)
- Windows (x86_64 and ARM64)

### From Source

```bash
cargo install draftkit
```

Or build from source:

```bash
git clone https://github.com/lovelesslabs/draftkit.git
cd draftkit
cargo install --path crates/draftkit
```

### Shell Completions

Shell completions are included in release archives and Homebrew installs. For manual installation, see [Shell Completions](#shell-completions) below.

## Usage

```bash
# Show version and build information
draftkit info

# JSON output for scripting
draftkit info --json

# Enable verbose output
draftkit --verbose <command>
```

## Configuration

Configuration files are discovered automatically in order of precedence (highest first):

1. `.draftkit.<ext>` in current directory or any parent
2. `draftkit.<ext>` in current directory or any parent
3. `~/.config/draftkit/config.<ext>` (user config)

**Supported formats:** TOML, YAML, JSON, JSON5, INI (extensions: `.toml`, `.yaml`, `.yml`, `.json`, `.json5`, `.ini`)

Values from higher-precedence files override lower ones. Missing files are silently ignored.

See the example configurations in the repository root for templates.

### Example Configuration

**TOML** (`~/.config/draftkit/config.toml`):
```toml
log_level = "info"
```

**YAML** (`~/.config/draftkit/config.yaml`):
```yaml
log_level: info
```

**JSON** (`~/.config/draftkit/config.json`):
```json
{
  "log_level": "info"
}
```

### Configuration Options

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `log_level` | `debug`, `info`, `warn`, `error` | `info` | Minimum log level to display |
| `log_dir` | path | platform default | Directory for JSONL log files |
| `otel_endpoint` | URL | unset | Enables OTel export when set |

## Logging

Logs are written as **JSONL** to a daily-rotated file.
Rotation is date-suffixed (e.g. `draftkit.jsonl.2026-01-06`).

Default log path (first writable wins):

1. `/var/log/draftkit.jsonl` (Unix only, requires write access)
2. OS user data directory (e.g. `~/.local/share/draftkit/logs/draftkit.jsonl`)

Overrides:

- `APP_LOG_PATH` — full file path
- `APP_LOG_DIR` — directory (file name defaults to `draftkit.jsonl`)
- `APP_ENV` — environment tag (default: `dev`)
- Config file keys: `log_dir` and `otel_endpoint`

OpenTelemetry tracing is **opt-in** when `OTEL_EXPORTER_OTLP_ENDPOINT` or `otel_endpoint` is set.

## Shell Completions

Shell completions are included in the release archives. To install manually:

**Bash**
```bash
draftkit completions bash > ~/.local/share/bash-completion/completions/draftkit
```

**Zsh**
```bash
draftkit completions zsh > ~/.zfunc/_draftkit
```

**Fish**
```bash
draftkit completions fish > ~/.config/fish/completions/draftkit.fish
```

**PowerShell**
```powershell
draftkit completions powershell > $PROFILE.CurrentUserAllHosts
```

## Development

This project uses a workspace layout with multiple crates:

```
crates/
├── draftkit/       # CLI binary
└── draftkit-core/  # Core library (config, errors)
```

### Prerequisites

- Rust {{msrv}}+ (2024 edition)
- [just](https://github.com/casey/just) (task runner)
- [cargo-nextest](https://nexte.st/) (test runner)

### Quick Start

```bash
# List available tasks
just --list

# Run full check suite (format, lint, test)
just check

# Run tests only
just test

# Run with coverage
just cov
```

### Build Tasks

| Command | Description |
|---------|-------------|
| `just check` | Format, lint, and test |
| `just fmt` | Format code with rustfmt |
| `just clippy` | Run clippy lints |
| `just test` | Run tests with nextest |
| `just doc-test` | Run documentation tests |
| `just cov` | Generate coverage report |

### xtask Commands

The project includes an xtask crate for build automation:

```bash
# Generate man pages
cargo xtask man

# Generate shell completions
cargo xtask completions

# Generate for specific shell
cargo xtask completions --shell zsh
```

## Architecture

### Crate Organization

- **draftkit** - The CLI binary. Handles argument parsing, command dispatch, and user interaction.
- **draftkit-core** - The core library. Contains configuration loading, error types, and shared functionality.

### Error Handling

- Libraries use `thiserror` for structured error types
- Binaries use `anyhow` for flexible error propagation
- All errors include context for debugging

### Configuration System

The `ConfigLoader` provides flexible configuration discovery:

```rust
use draftkit_core::config::{Config, ConfigLoader};

let config = ConfigLoader::new()
    .with_project_search(std::env::current_dir()?)
    .with_user_config(true)
    .load()?;
```

Features:
- Walks up directory tree looking for config files
- Stops at repository boundaries (`.git` by default)
- Merges multiple config sources with clear precedence
- Supports explicit file paths for testing

## CI/CD

This project uses GitHub Actions for continuous integration:

- **Build & Test** - Runs on every push and PR
- **MSRV Check** - Verifies minimum supported Rust version
- **Clippy** - Enforces lint rules
- **Coverage** - Tracks test coverage

### Release Process

Releases are automated via [cargo-dist](https://opensource.axo.dev/cargo-dist/) and [cocogitto](https://docs.cocogitto.io/):

1. Commits follow [Conventional Commits](https://www.conventionalcommits.org/)
2. Version bumps are calculated from commit history
3. Binaries are built for all supported platforms
4. Homebrew formula is updated automatically

To create a release:

```bash
# Bump version based on conventional commits
cog bump --auto

# Or specify version type
cog bump --patch
cog bump --minor
cog bump --major
```

### Dependabot

This project uses Dependabot for security monitoring, but **not** for automatic pull requests. Instead:

1. Dependabot scans for vulnerabilities in dependencies
2. A weekly GitHub Actions workflow converts alerts into **issues**
3. Maintainers review and address updates manually

This approach provides:
- Full control over when and how dependencies are updated
- Opportunity to batch related updates together
- Time to test updates before merging
- Cleaner git history without automated PR noise

Security alerts appear as issues labeled `dependabot-alert`.

## Contributing

Contributions welcome! Please see [AGENTS.md](AGENTS.md) for development conventions.

### Commit Messages

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

### Code Style

- Rust 2024 edition
- `#![deny(unsafe_code)]` - Safe Rust only
- Follow `rustfmt` defaults
- Keep clippy clean

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

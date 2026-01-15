# Scripts

Shell scripts for data collection, benchmarking, and development workflows.

## Data Collection

### tailwindplus-collector.sh

Full archival pipeline for TailwindPlus content. Downloads all UI components,
templates, and supporting assets in a single run.

```bash
# Basic usage - v4 only (outputs to cache/YYYY-MM-DD/)
./scripts/tailwindplus-collector.sh

# Don't include Tailwind CSS v3 formats (reduces download time)
./scripts/tailwindplus-collector.sh --v4-only

# With suffix for multiple pulls per day (outputs to cache/YYYY-MM-DD-test/)
./scripts/tailwindplus-collector.sh test

# Resume from last completed step after failure
./scripts/tailwindplus-collector.sh --resume

# Combine flags
./scripts/tailwindplus-collector.sh --v4-only --resume test
```

**Options:**
| Flag | Description |
|------|-------------|
| `--v4-only` | Don't include Tailwind CSS v3 formats (9 formats instead of 18) |
| `--resume` | Skip already-completed steps (uses `.progress` file) |
| `[suffix]` | Optional label appended to date directory (on days you're downloading more than once) |

**Prerequisites:**
- Valid TailwindPlus subscription
- `jq`, `curl`, `unzip`
- *optional* 1Password CLI (`op`) with TailwindPlus credentials stored

**Pipeline Steps:**

> [!NOTE]
> You don't need to run `tailwindplus-auth.sh`, `tailwindplus-collector.sh` does that for you.

| Step | Description |
|------|-------------|
| 1 | Authenticate via `tailwindplus-auth.sh` |
| 2 | Fetch portal data (template list, auth info) |
| 3 | Gather all 93 subcategory URLs from 3 products |
| 4 | Extract format UUID for server-side format switching |
| 5 | Download UI components for each format variant (9 or 18 formats × 93 subcategories) |
| 6 | Download Elements npm package and documentation |
| 7 | Download Catalyst UI Kit |
| 8 | Download all 13 template kits |
| 9 | Generate NDJSON files (merged light/dark/system per component) |
| 10 | Generate component index |
| 11 | Generate manifest with provenance metadata |

**Output Structure:**

```
cache/YYYY-MM-DD[-suffix]/
├── manifest.json           # Provenance: who downloaded, when, versions
├── portal.json             # Raw TailwindPlus portal data
├── marketing.json          # Product index pages
├── application-ui.json
├── ecommerce.json
├── all-subcategory-urls.txt
│
├── raw/                    # Per-format merged component data
│   ├── react-v3-*.json     # Tailwind CSS v3 formats
│   ├── react-v4-*.json     # Tailwind CSS v4 formats
│   ├── vue-v3-*.json
│   ├── vue-v4-*.json
│   ├── html-v3-*.json
│   ├── html-v4-*.json
│   └── <format>/           # Raw per-subcategory responses
│       └── *.json
│
├── data/
│   └── components/         # NDJSON for embedding generation
│       ├── react-v3.ndjson
│       ├── react-v4.ndjson
│       ├── vue-v3.ndjson
│       ├── vue-v4.ndjson
│       ├── html-v3.ndjson
│       └── html-v4.ndjson
│   └── component-index.json
│
├── elements/
│   └── src/                # @tailwindplus/elements npm package
│
├── kits/
│   ├── catalyst/           # Extracted Catalyst UI Kit
│   │   ├── typescript/
│   │   └── javascript/
│   ├── oatmeal.zip         # Template kit archives
│   ├── oatmeal.json        # Template metadata
│   ├── spotlight.zip
│   └── ...
│
└── docs/
    └── elements-llms.txt   # Elements documentation for AI
```

**Manifest Format:**

```json
{
  "downloaded_at": "2026-01-12T15:30:00Z",
  "downloaded_by": "user@example.com",
  "suffix": "test",
  "versions": {
    "tailwind": "4.1",
    "elements": "1.0.22",
    "inertia": "abc123..."
  },
  "counts": {
    "components": 657,
    "formats": 9,
    "kits": 13,
    "templates_available": 13
  },
  "templates": [
    {"name": "commit", "changelog_date": "2025-07-29", "file_mtime": "2025-12-23"},
    {"name": "spotlight", "changelog_date": "2025-11-18", "file_mtime": "2025-11-18"}
  ],
  "data_sources": [...]
}
```

**Tracking Multiple Pulls:**

When running multiple times per day, the script writes a tracking file:

```
cache/.2026-01-12    # Contains the suffix (or empty for default)
```

This allows scripts to find the most recent pull for a given date.


**Resumability:**

The script tracks completed steps in `$OUT_DIR/.progress`. If a step fails, use
`--resume` to restart from where it left off:

```bash
# First run fails at step 8
./scripts/tailwindplus-collector.sh
# ... fails downloading template kit

# Resume from step 8
./scripts/tailwindplus-collector.sh --resume
```

Each format and template kit is tracked individually, so partial downloads resume
at the granular level.

---

### tailwindplus-auth.sh

Authenticates with TailwindPlus using 1Password credentials. Handles Laravel's
CSRF token flow and Inertia.js session management.

```bash
# Usage (called automatically by archive script)
./scripts/tailwindplus-auth.sh [cookie-file]

# Default cookie file: cache/twp-cookies.txt
```

**Authentication Flow:**

1. Retrieve credentials from 1Password (`op item get`)
2. GET login page to obtain XSRF-TOKEN cookie
3. POST credentials to `/login` with CSRF token
4. Verify authentication via Inertia response
5. Save session cookies in Netscape format

**1Password Item Structure:**

The script looks for a 1Password item with:
- `username` field: email address
- `password` field: account password

Item is matched by searching for "tailwind" in 1Password.

**Cookie File Format:**

Uses Netscape cookie format compatible with `curl -b`:

```
# Netscape HTTP Cookie File
.tailwindcss.com	TRUE	/	TRUE	0	XSRF-TOKEN	...
.tailwindcss.com	TRUE	/	TRUE	0	tailwindcsscom_session	...
```

---

### inertia-version.sh

Extracts the current Inertia.js version hash from TailwindPlus. This version
must be sent with every API request to avoid 409 conflicts.

```bash
./scripts/inertia-version.sh
# Output: c8ab3c46a84c548dcade2288fa4b34ec
```

**How It Works:**

1. Check for cached `cache/data-page.json`
2. If not cached, fetch `/plus` HTML page
3. Extract `data-page` attribute from HTML
4. Decode HTML entities and parse as JSON
5. Return `.version` field

**Caching:**

The `data-page.json` is cached to avoid repeated HTML parsing. Delete this
file to force a refresh:

```bash
rm cache/data-page.json
```

---

## Benchmarking

### bench-cli.sh

CLI benchmarks using [hyperfine](https://github.com/sharkdp/hyperfine).
Measures end-to-end performance including startup time, argument parsing, and I/O.

```bash
# Full benchmark suite
./scripts/bench-cli.sh

# Quick mode (fewer runs)
./scripts/bench-cli.sh --quick
```

**Prerequisites:**

```bash
brew install hyperfine
# or
cargo install hyperfine
```

**Output:**

Results are saved to `bench-reports/` as JSON files for tracking performance
over time.

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CACHE_DIR` | `./cache` | Base directory for downloaded data |
| `COOKIE_FILE` | `$CACHE_DIR/twp-cookies.txt` | Cookie storage path |

---

## Troubleshooting

### "These credentials do not match our records"

1. Verify 1Password item has correct credentials
2. Ensure `--reveal` flag is used for password field
3. Check that password is not a placeholder (should be ~16 chars, not 65)

### HTTP 419 "Page Expired"

CSRF token mismatch. The `urldecode()` function must properly decode the
XSRF-TOKEN cookie before sending in the header. Check that `%3D` becomes `=`.

### HTTP 409 "Conflict"

Inertia version mismatch. Delete cached version and retry:

```bash
rm cache/data-page.json
./scripts/tailwindplus-collector.sh
```

### Empty or missing template downloads

Some templates may require specific subscription tiers. The script logs
failures but continues with remaining downloads.

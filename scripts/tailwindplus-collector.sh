#!/usr/bin/env bash
# TailwindPlus Collector
# Downloads: UI components (657), Elements docs, Catalyst kit, and 13 template kits
# Copies: Tailwind CSS v3/v4 documentation from docs/ to cache
#
# Usage: tailwindplus-archive.sh [options] [suffix]
#   --v4-only   Do not include Tailwind CSS v3 formats (default: v3 & v4)
#   --resume    Resume from last completed step
#   suffix      Optional label appended to date dir (e.g., "test" → cache/2026-01-12-test)
set -e

# Parse arguments
WITH_V3=true
RESUME=false
SUFFIX=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --v4-only)
      WITH_V3=false
      shift
      ;;
    --resume)
      RESUME=true
      shift
      ;;
    -*)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
    *)
      SUFFIX="$1"
      shift
      ;;
  esac
done

DATE=$(date +%Y-%m-%d)
DATETIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CACHE_DIR="${CACHE_DIR:-$PROJECT_ROOT/cache}"
COOKIE_FILE="$CACHE_DIR/twp-cookies.txt"

# Build output directory name with optional suffix
if [[ -n "$SUFFIX" ]]; then
  OUT_DIR="${CACHE_DIR}/${DATE}-${SUFFIX}"
else
  OUT_DIR="${CACHE_DIR}/${DATE}"
fi
RAW_DIR="${OUT_DIR}/raw"
KITS_DIR="${OUT_DIR}/kits"
DOCS_DIR="${OUT_DIR}/docs"

# formatting and color
boldred()     { printf '\033[1;31m%s\033[0m' "$*"; }
boldgreen()   { printf '\033[1;32m%s\033[0m' "$*"; }
OK=$(boldgreen "✓")
FAIL=$(boldred "✗")

# Convert absolute path to relative (./path)
relpath() {
  echo ".${1#$PROJECT_ROOT}"
}

# UI component format variants
if [[ "$WITH_V3" == "true" ]]; then
  FORMATS=(
    react-v3-system react-v3-light react-v3-dark
    react-v4-system react-v4-light react-v4-dark
    vue-v3-system vue-v3-light vue-v3-dark
    vue-v4-system vue-v4-light vue-v4-dark
    html-v3-system html-v3-light html-v3-dark
    html-v4-system html-v4-light html-v4-dark
  )
  VERSIONS=(v3 v4)
else
  FORMATS=(
    react-v4-system react-v4-light react-v4-dark
    vue-v4-system vue-v4-light vue-v4-dark
    html-v4-system html-v4-light html-v4-dark
  )
  VERSIONS=(v4)
fi

# Template kit slugs - populated from portal.json after step 2
KITS=()

HTTP_CODE=""
XSRF=""
INERTIA_VERSION=""
FORMAT=""
LINES=0

urldecode() {
  # Convert %XX sequences to characters
  # Note: bash ${s//%/\\x} doesn't work because % is special in parameter expansion
  local decoded
  decoded=$(printf '%s' "$1" | sed 's/%/\\x/g')
  printf '%b' "$decoded"
}

# Progress tracking for resumability
PROGRESS_FILE=""  # Set after OUT_DIR is known

step_done() {
  local step="$1"
  [[ -f "$PROGRESS_FILE" ]] && grep -qxF "$step" "$PROGRESS_FILE"
}

mark_step() {
  local step="$1"
  echo "$step" >> "$PROGRESS_FILE"
}

skip_if_done() {
  local step="$1"
  if [[ "$RESUME" == "true" ]] && step_done "$step"; then
    echo "--- Step $step: SKIPPED (already complete) ---"
    return 0
  fi
  return 1
}

get_xsrf() {
  raw=$(awk -F'\t' '$6=="XSRF-TOKEN"{print $7; exit}' "$COOKIE_FILE")
  [[ -n "$raw" ]] || { echo "get_xsrf: XSRF-TOKEN not found in $COOKIE_FILE" >&2; exit 1; }

  urldecode "$raw"
}

simple_curl() {
  local url="$1"
  local saveas="$2"

  if [[ -n "$XSRF" ]]; then
    XSRF=$(get_xsrf)
  fi
  HTTP_CODE=$(curl -sS -L -w "%{http_code}" -b "$COOKIE_FILE" -c "$COOKIE_FILE" -o "$OUT_DIR/$saveas" "$url")

  if [[ "$HTTP_CODE" == "200" ]]; then
    echo "${OK} saved to: ${saveas}"
  else
    # handle failure codes
    echo "${FAIL} Failed to fetch ${url} (HTTP ${HTTP_CODE})" >&2
    rm -f "$OUT_DIR/$saveas"
    exit 1
  fi
}

inertia_fetch() {
  local url="$1"
  local saveas="$2"

  if [ -z "$INERTIA_VERSION" ]; then
    INERTIA_VERSION=$("$SCRIPT_DIR/inertia-version.sh")
  fi

  HTTP_CODE=$(curl -sS -L -w "%{http_code}" -b "$COOKIE_FILE" -c "$COOKIE_FILE" -H 'x-inertia: true' -H "x-inertia-version: $INERTIA_VERSION" -H 'x-requested-with: XMLHttpRequest' -o "$OUT_DIR/$saveas" "$url")

  if [[ "$HTTP_CODE" == "200" ]]; then
    echo "${OK} saved to: ${saveas}"
  else
    # handle failure codes
    echo "${FAIL} Failed to fetch ${url} (HTTP ${HTTP_CODE})" >&2
    rm -f "$OUT_DIR/$saveas"
    exit 1
  fi
}

fetch_file() {
  local url="$1"
  local saveas="$2"

  if [ -z "$XSRF" ]; then
    XSRF=$(get_xsrf)
  fi
  if [ -z "$INERTIA_VERSION" ]; then
    INERTIA_VERSION=$("$SCRIPT_DIR/inertia-version.sh")
  fi

  HTTP_CODE=$(curl -sS -w "%{http_code}" -b "$COOKIE_FILE" \
    -H "accept: text/html, application/xhtml+xml, application/json" \
    -H 'x-inertia: true' \
    -H "x-inertia-version: $INERTIA_VERSION" \
    -H 'x-requested-with: XMLHttpRequest' \
    -H "x-xsrf-token: $XSRF" \
    -H 'referer: https://tailwindcss.com/plus/ui-blocks/marketing' \
    -H 'sec-ch-ua: "Google Chrome";v="143", "Chromium";v="143", "Not A(Brand";v="24"' \
    -H 'sec-ch-ua-mobile: ?0' \
    -H 'sec-ch-ua-platform: "macOS"' \
    -H 'sec-fetch-dest: empty' \
    -H 'sec-fetch-mode: cors' \
    -H 'sec-fetch-site: same-origin' \
    -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36' \
    -H 'accept-language: en-US,en;q=0.9,fr;q=0.8' \
    -H 'cache-control: no-cache' \
    -H 'pragma: no-cache' \
    -H 'priority: u=1, i' \
    -o "$OUT_DIR/$saveas" "$url")

  if [[ "$HTTP_CODE" == "200" ]]; then
    LINES=$(wc -l < "$OUT_DIR/$saveas" | tr -d ' ')
    echo "${OK} saved to: ${saveas} (${LINES} lines)"
  else
    # handle failure codes
    echo "${FAIL} Failed to fetch ${url} (HTTP ${HTTP_CODE})" >&2
    rm -f "$OUT_DIR/$saveas"
    exit 1
  fi
}

echo "=== TailwindPlus Collector ==="
echo "Output directory: $OUT_DIR"
echo "Formats: ${#FORMATS[@]} (${VERSIONS[*]})"
[[ "$RESUME" == "true" ]] && echo "Mode: RESUME"
echo ""

# make sure out directory exists
mkdir -p "$OUT_DIR"
mkdir -p "$RAW_DIR"
mkdir -p "$KITS_DIR"
mkdir -p "$DOCS_DIR"

# Initialize progress tracking
PROGRESS_FILE="$OUT_DIR/.progress"

# Step 0: Copy Tailwind CSS documentation to cache
if ! skip_if_done "0-docs"; then
  echo "--- Step 0: Copying Tailwind CSS documentation ---"
  TAILWIND_DOCS_SRC="$PROJECT_ROOT/docs"
  TAILWIND_DOCS_DST="$DOCS_DIR/tailwind"

  mkdir -p "$TAILWIND_DOCS_DST/v3"
  mkdir -p "$TAILWIND_DOCS_DST/v4"

  # Copy v3 docs (excluding README)
  v3_count=0
  for f in "$TAILWIND_DOCS_SRC/tailwind-v3"/*.md; do
    if [[ -f "$f" && "$(basename "$f")" != "README.md" ]]; then
      cp "$f" "$TAILWIND_DOCS_DST/v3/"
      v3_count=$((v3_count + 1))
    fi
  done

  # Copy v4 docs (excluding README)
  v4_count=0
  for f in "$TAILWIND_DOCS_SRC/tailwind-v4"/*.md; do
    if [[ -f "$f" && "$(basename "$f")" != "README.md" ]]; then
      cp "$f" "$TAILWIND_DOCS_DST/v4/"
      v4_count=$((v4_count + 1))
    fi
  done

  echo "${OK} Tailwind docs: v3 ($v3_count files), v4 ($v4_count files)"
  mark_step "0-docs"
  echo ""
fi

# Step 1: Authenticate
if ! skip_if_done "1-auth"; then
  echo "--- Step 1: Authenticating ---"
  "$SCRIPT_DIR/tailwindplus-auth.sh" "$COOKIE_FILE"
  mark_step "1-auth"
  echo ""
fi

# Step 2: Get data-page.json for template list and auth info
if ! skip_if_done "2-portal"; then
  echo "--- Step 2: Fetching portal data ---"
  XSRF=$(get_xsrf)
  INERTIA_VERSION=$("$SCRIPT_DIR/inertia-version.sh")

  # Fetch the main Plus page to get auth user info and template list
  curl -sS -b "$COOKIE_FILE" \
    -H 'x-inertia: true' \
    -H "x-inertia-version: $INERTIA_VERSION" \
    -H "x-xsrf-token: $XSRF" \
    -H 'x-requested-with: XMLHttpRequest' \
    'https://tailwindcss.com/plus' > "$OUT_DIR/portal.json"

  mark_step "2-portal"
  echo ""
fi

# Extract auth user for manifest (always needed for later steps)
XSRF=$(get_xsrf)
INERTIA_VERSION=$("$SCRIPT_DIR/inertia-version.sh")
AUTH_EMAIL=$(jq -r '.props.auth.user.email // "unknown"' "$OUT_DIR/portal.json")
TAILWIND_VERSION=$(jq -r '.props.tailwindVersion // "unknown"' "$OUT_DIR/portal.json")
TEMPLATE_COUNT=$(jq '.props.templates | length' "$OUT_DIR/portal.json")

# Extract template slugs from portal data (from URL, not name)
mapfile -t KITS < <(jq -r '.props.templates[].url | split("/")[-1]' "$OUT_DIR/portal.json")

echo "${OK} Authenticated as: $AUTH_EMAIL"
echo "${OK} Tailwind version: $TAILWIND_VERSION"
echo "${OK} Templates available: $TEMPLATE_COUNT"
echo ""

# Step 3: determine all subcategory urls
if ! skip_if_done "3-urls"; then
  echo "--- Step 3: Gathering UI component urls ---"
  fetch_file "https://tailwindcss.com/plus/ui-blocks/marketing" "marketing.json"
  fetch_file "https://tailwindcss.com/plus/ui-blocks/application-ui" "application-ui.json"
  fetch_file "https://tailwindcss.com/plus/ui-blocks/ecommerce" "ecommerce.json"

  # generate all-subcategory-urls.txt from all product pages
  # Note: structure is .props.product.categories
  {
    jq -r '.props.product.categories[].subcategories[].url' "$OUT_DIR/marketing.json"
    jq -r '.props.product.categories[].subcategories[].url' "$OUT_DIR/application-ui.json"
    jq -r '.props.product.categories[].subcategories[].url' "$OUT_DIR/ecommerce.json"
  } | sort -u > "$OUT_DIR/all-subcategory-urls.txt"

  echo "${OK} Generated all-subcategory-urls.txt ($(wc -l < "$OUT_DIR/all-subcategory-urls.txt" | tr -d ' ') URLs)"
  mark_step "3-urls"
  echo ""
fi

# Step 4: Extract format UUID from first subcategory
if ! skip_if_done "4-uuid"; then
  echo "--- Step 4: Extracting format UUID ---"
  FIRST_URL=$(head -1 "$OUT_DIR/all-subcategory-urls.txt")
  FIRST_SLUG=$(echo "$FIRST_URL" | sed 's#.*/plus/ui-blocks/##; s#[^a-zA-Z0-9]#_#g')
  fetch_file "$FIRST_URL" "first-subcategory.json"

  FORMAT_UUID=$(jq -r '.props.subcategory.components[0].uuid' "$OUT_DIR/first-subcategory.json")
  if [[ -z "$FORMAT_UUID" || "$FORMAT_UUID" == "null" ]]; then
    echo "${FAIL} Could not extract format UUID from first subcategory" >&2
    exit 1
  fi
  echo "$FORMAT_UUID" > "$OUT_DIR/.format-uuid"
  echo "${OK} Format UUID: $FORMAT_UUID"
  mark_step "4-uuid"
  echo ""
else
  FORMAT_UUID=$(cat "$OUT_DIR/.format-uuid" 2>/dev/null || echo "")
  if [[ -z "$FORMAT_UUID" ]]; then
    echo "${FAIL} Cannot resume: missing .format-uuid file" >&2
    exit 1
  fi
fi

# Step 5: Download all subcategories for each format
set_format() {
  local fmt="$1"
  local response http_code body

  response=$(curl -sS -w '\n%{http_code}' -X PUT 'https://tailwindcss.com/plus/ui-blocks/language' \
    -b "$COOKIE_FILE" \
    -c "$COOKIE_FILE" \
    -H 'content-type: application/json' \
    -H 'origin: https://tailwindcss.com' \
    -H 'x-inertia: true' \
    -H "x-inertia-version: $INERTIA_VERSION" \
    -H 'x-requested-with: XMLHttpRequest' \
    -H "x-xsrf-token: $XSRF" \
    --data-raw "{\"uuid\":\"$FORMAT_UUID\",\"snippet_lang\":\"$fmt\"}")

  http_code=$(echo "$response" | tail -1)
  body=$(echo "$response" | sed '$d')

  case "$http_code" in
    200|303)
      echo "${OK} set-format: $fmt"
      return 0
      ;;
    409)
      echo "${FAIL} set-format: 409 Inertia version mismatch" >&2
      return 1
      ;;
    401|403|419)
      echo "${FAIL} set-format: $http_code auth error" >&2
      return 1
      ;;
    *)
      echo "${FAIL} set-format: unexpected $http_code" >&2
      echo "$body" >&2
      return 1
      ;;
  esac
}

fetch_all_subcategories() {
  local outdir="$1"
  local url slug

  mkdir -p "$outdir"

  # Refresh tokens
  XSRF=$(get_xsrf)

  # Build curl arguments for all URLs
  local curl_args=()
  while IFS= read -r url; do
    slug=$(echo "$url" | sed 's#.*/plus/ui-blocks/##; s#[^a-zA-Z0-9]#_#g')
    curl_args+=("-o" "$outdir/${slug}.json" "$url")
  done < "$OUT_DIR/all-subcategory-urls.txt"

  # Single curl invocation with HTTP/2 multiplexing
  curl --http2 \
    --parallel --parallel-immediate --parallel-max 10 \
    -sS \
    -b "$COOKIE_FILE" \
    -c "$COOKIE_FILE" \
    -H "accept: text/html, application/xhtml+xml, application/json" \
    -H "x-inertia: true" \
    -H "x-inertia-version: $INERTIA_VERSION" \
    -H "x-xsrf-token: $XSRF" \
    -H "x-requested-with: XMLHttpRequest" \
    --write-out "%output{>>$OUT_DIR/batch-run.ndjson}%{json}\n" \
    "${curl_args[@]}"
}

merge_subcategories() {
  local rawdir="$1"

  if [[ ! -d "$rawdir" ]]; then
    echo "merge_subcategories: directory not found: $rawdir" >&2
    return 1
  fi

  # Find all JSON files and merge them with jq
  find "$rawdir" -name "*.json" -type f -print0 | \
    xargs -0 jq -s '
      # Start with empty object, merge each file
      reduce .[] as $file ({};
        # Extract hierarchy from each file
        ($file.props.subcategory) as $sub |
        ($sub.category) as $cat |
        ($cat.product) as $prod |

        # Build nested structure: Product > Category > Subcategory > Component
        .[$prod.name] //= {} |
        .[$prod.name][$cat.name] //= {} |
        .[$prod.name][$cat.name][$sub.name] //= {} |

        # Add each component
        reduce $sub.components[] as $comp (.;
          .[$prod.name][$cat.name][$sub.name][$comp.name] = {
            uuid: $comp.uuid,
            snippet: {
              code: $comp.snippet.code,
              name: $comp.snippet.name,
              language: $comp.snippet.language,
              version: $comp.snippet.version,
              mode: $comp.snippet.mode,
              supportsDarkMode: $comp.snippet.supportsDarkMode,
              preview: $comp.snippet.preview
            }
          }
        )
      )
    '
}

# Step 5: Download all subcategories for each format
echo "--- Step 5: Downloading all formats ---"
echo "Formats: ${#FORMATS[@]}"
echo "Subcategories: $(wc -l < "$OUT_DIR/all-subcategory-urls.txt" | tr -d ' ')"
echo ""

for fmt in "${FORMATS[@]}"; do
  if skip_if_done "5-format-$fmt"; then
    continue
  fi

  echo "--- Format: $fmt ---"

  # Refresh XSRF before format change
  XSRF=$(get_xsrf)

  # 1. Set format on server
  if ! set_format "$fmt"; then
    echo "${FAIL} Skipping $fmt due to format switch failure" >&2
    continue
  fi

  # 2. Fetch all subcategories with HTTP/2 multiplexing
  fetch_all_subcategories "$RAW_DIR/$fmt"

  # 3. Count downloaded files
  count=$(find "$RAW_DIR/$fmt" -name "*.json" -type f 2>/dev/null | wc -l | tr -d ' ')
  echo "${OK} Downloaded: $count files"

  # 4. Merge into single JSON
  merge_subcategories "$RAW_DIR/$fmt" > "$RAW_DIR/$fmt.json"
  size=$(du -h "$RAW_DIR/$fmt.json" | cut -f1)
  echo "${OK} Merged: $fmt.json ($size)"

  mark_step "5-format-$fmt"
  echo ""
done

echo "=== UI components download complete ==="
echo ""

# Step 6: Download Elements documentation and npm package
if ! skip_if_done "6-elements"; then
  echo "--- Step 6: Fetching Elements ---"
  ELEMENTS_URL="https://tailwindcss.com/plus/ui-blocks/documentation/llms.txt"

  # Download llms.txt documentation
  HTTP_CODE=$(curl -sS -w "%{http_code}" -b "$COOKIE_FILE" -o "$DOCS_DIR/elements-llms.txt" "$ELEMENTS_URL")
  if [[ "$HTTP_CODE" == "200" ]]; then
    LINES=$(wc -l < "$DOCS_DIR/elements-llms.txt" | tr -d ' ')
    echo "${OK} Elements docs: $LINES lines"
  else
    echo "${FAIL} Failed to fetch Elements docs (HTTP $HTTP_CODE)" >&2
  fi

  # Download @tailwindplus/elements npm package
  ELEMENTS_DIR="$OUT_DIR/elements"
  mkdir -p "$ELEMENTS_DIR"

  # Get the tarball directly from npm registry
  ELEMENTS_VERSION=$(npm view @tailwindplus/elements version 2>/dev/null || echo "")
  if [[ -n "$ELEMENTS_VERSION" ]]; then
    echo "$ELEMENTS_VERSION" > "$OUT_DIR/.elements-version"
    TARBALL_URL="https://registry.npmjs.org/@tailwindplus/elements/-/elements-${ELEMENTS_VERSION}.tgz"
    if curl -sS -o "$ELEMENTS_DIR/elements-${ELEMENTS_VERSION}.tgz" "$TARBALL_URL"; then
      # Extract the package
      tar -xzf "$ELEMENTS_DIR/elements-${ELEMENTS_VERSION}.tgz" -C "$ELEMENTS_DIR"
      mv "$ELEMENTS_DIR/package" "$ELEMENTS_DIR/src"
      rm "$ELEMENTS_DIR/elements-${ELEMENTS_VERSION}.tgz"
      file_count=$(find "$ELEMENTS_DIR/src" -type f | wc -l | tr -d ' ')
      echo "${OK} Elements npm: v${ELEMENTS_VERSION} ($file_count files)"
    else
      echo "${FAIL} Failed to download Elements npm package" >&2
    fi
  else
    echo "${FAIL} Could not determine Elements version from npm" >&2
  fi
  mark_step "6-elements"
  echo ""
fi

# Step 7: Download Catalyst UI Kit
if ! skip_if_done "7-catalyst"; then
  echo "--- Step 7: Fetching Catalyst UI Kit ---"
  CATALYST_URL="https://tailwindcss.com/plus/templates/catalyst/download"
  CATALYST_ZIP="$KITS_DIR/catalyst.zip"

  HTTP_CODE=$(curl -sS -L -w "%{http_code}" -b "$COOKIE_FILE" -o "$CATALYST_ZIP" "$CATALYST_URL")
  if [[ "$HTTP_CODE" == "200" ]] && file "$CATALYST_ZIP" | grep -q "Zip archive"; then
    # Extract to kits/catalyst/
    unzip -q -o "$CATALYST_ZIP" -d "$KITS_DIR/.catalyst-extract"
    EXTRACTED=$(find "$KITS_DIR/.catalyst-extract" -maxdepth 1 -type d -name "catalyst*" | head -1)
    if [[ -d "$EXTRACTED" ]]; then
      rm -rf "$KITS_DIR/catalyst"
      mv "$EXTRACTED" "$KITS_DIR/catalyst"
      rm -rf "$KITS_DIR/.catalyst-extract"
      rm -f "$CATALYST_ZIP"
      TS_COUNT=$(find "$KITS_DIR/catalyst" -name "*.tsx" 2>/dev/null | wc -l | tr -d ' ')
      echo "${OK} Catalyst: $TS_COUNT TypeScript components"
    else
      echo "${FAIL} Could not find Catalyst in ZIP" >&2
    fi
  else
    echo "${FAIL} Failed to download Catalyst (HTTP $HTTP_CODE)" >&2
    rm -f "$CATALYST_ZIP"
  fi
  mark_step "7-catalyst"
  echo ""
fi

# Step 8: Download template kits
echo "--- Step 8: Fetching template kits ---"
XSRF=$(get_xsrf)

# Helper to extract latest date from CHANGELOG.md inside a ZIP
extract_changelog_date() {
  local zipfile="$1"
  # Find CHANGELOG.md files (root or nested) and extract dates
  # Try both patterns: root level and nested
  {
    unzip -p "$zipfile" 'CHANGELOG.md' 2>/dev/null
    unzip -p "$zipfile" '*/CHANGELOG.md' 2>/dev/null
  } | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sort -r | head -1
}

# Helper to extract most recent file modification date from ZIP
extract_latest_mtime() {
  local zipfile="$1"
  # unzip -l format: Length Date Time Name
  # Date is in MM-DD-YYYY or YYYY-MM-DD format depending on unzip version
  # We normalize to YYYY-MM-DD for sorting
  unzip -l "$zipfile" 2>/dev/null | \
    awk 'NF>=4 && $2 ~ /[0-9]+-[0-9]+-[0-9]+/ {
      # Handle both MM-DD-YYYY and YYYY-MM-DD formats
      split($2, d, "-")
      if (length(d[1]) == 4) {
        # Already YYYY-MM-DD
        print $2
      } else if (length(d[3]) == 4) {
        # MM-DD-YYYY -> YYYY-MM-DD
        printf "%s-%s-%s\n", d[3], d[1], d[2]
      }
    }' | sort -r | head -1
}

# Helper to fetch kit metadata - tries /kits/ first, falls back to /templates/
fetch_kit_metadata() {
  local kit="$1"
  local kit_json=""

  # Try /kits/ first (newer templates like Catalyst)
  kit_json=$(curl -sS -b "$COOKIE_FILE" \
    -H 'x-inertia: true' \
    -H "x-inertia-version: $INERTIA_VERSION" \
    -H "x-xsrf-token: $XSRF" \
    -H 'x-requested-with: XMLHttpRequest' \
    "https://tailwindcss.com/plus/kits/$kit" 2>/dev/null)

  # Check if we got a valid response with download_url
  if echo "$kit_json" | jq -e '.props.product.download_url' >/dev/null 2>&1; then
    echo "$kit_json"
    return 0
  fi

  # Fall back to /templates/ (older templates)
  kit_json=$(curl -sS -b "$COOKIE_FILE" \
    -H 'x-inertia: true' \
    -H "x-inertia-version: $INERTIA_VERSION" \
    -H "x-xsrf-token: $XSRF" \
    -H 'x-requested-with: XMLHttpRequest' \
    "https://tailwindcss.com/plus/templates/$kit" 2>/dev/null)

  echo "$kit_json"
}

for kit in "${KITS[@]}"; do
  if skip_if_done "8-kit-$kit"; then
    continue
  fi

  # Fetch kit metadata (tries /kits/ then /templates/)
  kit_json=$(fetch_kit_metadata "$kit")

  download_url=$(echo "$kit_json" | jq -r '.props.product.download_url // empty')

  if [[ -z "$download_url" ]]; then
    echo "${FAIL} $kit: no download URL found"
    continue
  fi

  # Download the kit ZIP
  HTTP_CODE=$(curl -sS -L -w "%{http_code}" -b "$COOKIE_FILE" -o "$KITS_DIR/$kit.zip" "$download_url")

  if [[ "$HTTP_CODE" == "200" ]] && file "$KITS_DIR/$kit.zip" | grep -q "Zip archive"; then
    size=$(du -h "$KITS_DIR/$kit.zip" | cut -f1)

    # Extract dates
    changelog_date=$(extract_changelog_date "$KITS_DIR/$kit.zip")
    file_mtime=$(extract_latest_mtime "$KITS_DIR/$kit.zip")

    # Save kit metadata with both dates (null-safe for missing fields)
    echo "$kit_json" | jq \
      --arg changelog_date "${changelog_date:-null}" \
      --arg file_mtime "${file_mtime:-null}" \
      '.props.product | {
        name,
        type,
        technologies,
        themes: [(.themes // [])[] | .name],
        changelog_date: (if $changelog_date == "null" then null else $changelog_date end),
        file_mtime: (if $file_mtime == "null" then null else $file_mtime end)
      }' \
      > "$KITS_DIR/$kit.json" 2>/dev/null || echo '{}' > "$KITS_DIR/$kit.json"

    # Extract the template kit for intelligence analysis
    unzip -q -o "$KITS_DIR/$kit.zip" -d "$KITS_DIR/.${kit}-extract"
    # Find the extracted directory (usually named after the kit)
    EXTRACTED=$(find "$KITS_DIR/.${kit}-extract" -maxdepth 1 -type d ! -name ".${kit}-extract" | head -1)
    if [[ -d "$EXTRACTED" ]]; then
      rm -rf "$KITS_DIR/$kit"
      mv "$EXTRACTED" "$KITS_DIR/$kit"
      rm -rf "$KITS_DIR/.${kit}-extract"
      tsx_count=$(find "$KITS_DIR/$kit" -name "*.tsx" 2>/dev/null | wc -l | tr -d ' ')
    else
      tsx_count=0
    fi

    # Show dates (highlight if they differ)
    if [[ -n "$changelog_date" && -n "$file_mtime" && "$changelog_date" != "$file_mtime" ]]; then
      echo "${OK} $kit: $size, $tsx_count tsx (changelog: $changelog_date, files: $file_mtime)"
    elif [[ -n "$changelog_date" ]]; then
      echo "${OK} $kit: $size, $tsx_count tsx (updated: $changelog_date)"
    else
      echo "${OK} $kit: $size, $tsx_count tsx"
    fi
    mark_step "8-kit-$kit"
  else
    echo "${FAIL} $kit: download failed (HTTP $HTTP_CODE)"
    rm -f "$KITS_DIR/$kit.zip"
  fi
done
echo ""

# Step 9: Generate NDJSON files (merging light/dark/system modes)
if ! skip_if_done "9-ndjson"; then
  echo "--- Step 9: Generating NDJSON files ---"
  NDJSON_DIR="$OUT_DIR/data/components"
  mkdir -p "$NDJSON_DIR"

  for version in "${VERSIONS[@]}"; do
    for framework in react vue html; do
    light_file="$RAW_DIR/${framework}-${version}-light.json"
    dark_file="$RAW_DIR/${framework}-${version}-dark.json"
    system_file="$RAW_DIR/${framework}-${version}-system.json"
    output_file="$NDJSON_DIR/${framework}-${version}.ndjson"

    if [[ ! -f "$light_file" ]]; then
      echo "${FAIL} Missing $light_file, skipping $framework-$version" >&2
      continue
    fi

    echo "Processing $framework-$version..."

    # Merge light/dark/system into NDJSON
    jq -c --slurpfile light "$light_file" \
          --slurpfile dark "$dark_file" \
          --slurpfile system "$system_file" \
          --arg version "$version" \
    '
    # Walk the nested structure and emit one line per component
    [
        $light[0] | paths(type == "object" and has("uuid") and has("snippet")) as $path |
        {
            path: $path,
            category: $path[0],
            subcategory: $path[1],
            sub_subcategory: $path[2],
            name: $path[3],
            uuid: (getpath($path) | .uuid)
        }
    ] | .[] |

    # Build the ID from the path
    . as $meta |
    ($meta.category + "/" + $meta.subcategory + "/" + $meta.sub_subcategory + "/" + $meta.name
        | gsub("[^a-zA-Z0-9/]"; "-") | ascii_downcase) as $id |

    # Get snippets from all three modes
    ($light[0] | getpath($meta.path) | .snippet) as $light_snippet |
    ($dark[0] | getpath($meta.path) | .snippet // null) as $dark_snippet |
    ($system[0] | getpath($meta.path) | .snippet // null) as $system_snippet |

    {
        id: $id,
        uuid: $meta.uuid,
        name: $meta.name,
        version: $version,
        category: $meta.category,
        subcategory: $meta.subcategory,
        sub_subcategory: $meta.sub_subcategory,
        light: $light_snippet,
        dark: $dark_snippet,
        system: $system_snippet
    }
    ' -n > "$output_file"

    line_count=$(wc -l < "$output_file" | tr -d ' ')
    file_size=$(du -h "$output_file" | cut -f1)
    echo "${OK} Created: $(relpath "$output_file") ($line_count components, $file_size)"
    done
  done
  mark_step "9-ndjson"
fi

# Step 10: Generate component index
if ! skip_if_done "10-index"; then
  echo ""
  echo "--- Step 10: Generating component index ---"
  NDJSON_DIR="$OUT_DIR/data/components"
  index_file="$OUT_DIR/data/component-index.json"

  # Build versions array as JSON
  versions_json=$(printf '%s\n' "${VERSIONS[@]}" | jq -R . | jq -s .)

  # Use react-v4 as the canonical source for metadata
  jq -s --argjson versions "$versions_json" '
  {
      generated_at: (now | strftime("%Y-%m-%dT%H:%M:%SZ")),
      component_count: length,
      frameworks: ["react", "vue", "html"],
      versions: $versions,

    # Category tree with counts
    categories: (
        group_by(.category) | map({
            name: .[0].category,
            subcategories: (
                group_by(.subcategory) | map({
                    name: .[0].subcategory,
                    sub_subcategories: (
                        group_by(.sub_subcategory) | map({
                            name: .[0].sub_subcategory,
                            count: length
                        })
                    )
                })
            )
        })
    ),

    # Component list (lightweight, no code)
    components: [.[] | {
        id,
        uuid,
        name,
        category,
        subcategory,
        sub_subcategory,
        has_dark_mode: (.dark != null),
        has_system_mode: (.system != null)
    }]
}
' "$NDJSON_DIR/react-v4.ndjson" > "$index_file"

  index_size=$(du -h "$index_file" | cut -f1)
  component_count=$(jq '.component_count' "$index_file")
  echo "${OK} Created: $(relpath "$index_file") ($component_count components, $index_size)"
  mark_step "10-index"
fi

# Step 11: Generate archive manifest with provenance
echo ""
echo "--- Step 11: Generating archive manifest ---"
manifest_file="$OUT_DIR/manifest.json"
NDJSON_DIR="$OUT_DIR/data/components"
index_file="$OUT_DIR/data/component-index.json"

# Count kits downloaded
kit_count=$(find "$KITS_DIR" -maxdepth 1 -name "*.zip" -type f 2>/dev/null | wc -l | tr -d ' ')

# Get Elements version (from saved file or current download)
elements_version=$(cat "$OUT_DIR/.elements-version" 2>/dev/null || echo "unknown")

# Get component count from index
component_count=$(jq '.component_count // 0' "$index_file" 2>/dev/null || echo "0")

# Collect template metadata (name + dates) from kit JSON files
template_info=$(find "$KITS_DIR" -maxdepth 1 -name "*.json" -type f -exec cat {} \; 2>/dev/null | \
  jq -s '[.[] | {name, changelog_date, file_mtime}]' 2>/dev/null || echo '[]')

# Build data sources list (reflects what was actually downloaded)
if [[ "$WITH_V3" == "true" ]]; then
  version_desc="v3/v4"
else
  version_desc="v4"
fi
data_sources=$(jq -n --arg v "$version_desc" '[
  "UI components (react/vue/html × \($v) × light/dark/system)",
  "Elements npm package (@tailwindplus/elements)",
  "Elements documentation (llms.txt)",
  "Catalyst UI Kit",
  "Template kits (13 themes)",
  "Tailwind CSS documentation (v3/v4)"
]')

jq -n \
  --arg downloaded_at "$DATETIME" \
  --arg downloaded_by_email "$AUTH_EMAIL" \
  --arg tailwind_version "$TAILWIND_VERSION" \
  --arg elements_version "$elements_version" \
  --arg inertia_version "$INERTIA_VERSION" \
  --arg suffix "${SUFFIX:-}" \
  --argjson component_count "$component_count" \
  --argjson format_count "${#FORMATS[@]}" \
  --argjson kit_count "$kit_count" \
  --argjson template_count "$TEMPLATE_COUNT" \
  --argjson templates "$template_info" \
  --argjson data_sources "$data_sources" \
'{
  downloaded_at: $downloaded_at,
  downloaded_by: $downloaded_by_email,
  suffix: (if $suffix == "" then null else $suffix end),
  versions: {
    tailwind: $tailwind_version,
    elements: $elements_version,
    inertia: $inertia_version
  },
  counts: {
    components: $component_count,
    formats: $format_count,
    kits: $kit_count,
    templates_available: $template_count
  },
  templates: $templates,
  data_sources: $data_sources
}' > "$manifest_file"

echo "${OK} Created: $(relpath "$manifest_file")"

# Write tracking file for this date (allows comparing multiple pulls per day)
tracking_file="$CACHE_DIR/.${DATE}"
echo "$SUFFIX" > "$tracking_file"
echo "${OK} Tracking: $(relpath "$tracking_file") → ${SUFFIX:-\"(default)\"}"

# Summary
echo ""
echo "=== Collection Complete ==="
ELEMENTS_DIR="$OUT_DIR/elements"
total_size=$(du -sh "$OUT_DIR" | cut -f1)
echo "Output:     $(relpath "$OUT_DIR") ($total_size)"
echo "Downloaded: $(date -r "$OUT_DIR/manifest.json" '+%Y-%m-%d %H:%M:%S') UTC"
echo "User:       $AUTH_EMAIL"
echo ""
echo "Contents:"
echo "  - UI components: $component_count (${#FORMATS[@]} format variants, ${VERSIONS[*]})"
echo "  - Template kits: $kit_count"
echo "  - Catalyst:      $(test -d "$KITS_DIR/catalyst" && echo "yes" || echo "no")"
echo "  - Elements npm:  $(test -d "$ELEMENTS_DIR/src" && echo "v${elements_version}" || echo "no")"
echo "  - Elements docs: $(test -f "$DOCS_DIR/elements-llms.txt" && echo "yes" || echo "no")"
tw_v3_count=$(find "$DOCS_DIR/tailwind/v3" -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
tw_v4_count=$(find "$DOCS_DIR/tailwind/v4" -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
echo "  - Tailwind docs: v3 ($tw_v3_count), v4 ($tw_v4_count)"
echo ""

# Step 12: Extract component metadata
echo ""
echo "--- Step 12: Extracting component metadata ---"
METADATA_SCRIPT="$SCRIPT_DIR/metadata.sh"
if [[ -x "$METADATA_SCRIPT" ]]; then
  "$METADATA_SCRIPT" "$OUT_DIR/data/components"
  echo "${OK} Metadata extraction complete"
else
  echo "⚠ Skipping metadata extraction (scripts/metadata.sh not found)"
fi

echo ""
echo "Next step: generate embeddings"
echo "  cargo run --release -p xtask -- gen-embeddings"

# Step 13: Update cache/current symlink
echo ""
echo "--- Updating cache/current symlink ---"
CURRENT_LINK="$CACHE_DIR/current"
TARGET_BASENAME=$(basename "$OUT_DIR")

# Remove existing symlink if present
if [[ -L "$CURRENT_LINK" ]]; then
  rm "$CURRENT_LINK"
fi

# Create relative symlink: cache/current -> YYYY-MM-DD[-suffix]
ln -s "$TARGET_BASENAME" "$CURRENT_LINK"
echo "${OK} cache/current -> $TARGET_BASENAME"

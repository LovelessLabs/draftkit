#!/usr/bin/env bash
# metadata.sh - Extract component metadata and strip source code from NDJSON files
#
# Usage: ./metadata.sh <input-dir>
#
# Processes all *-v*.ndjson files in the input directory:
# 1. Extracts metadata from source code (dependencies, tokens, tailwind features)
# 2. STRIPS source code from output (only metadata-safe fields remain)
#
# Output format (no source code - safe to distribute):
#   - id, uuid, name, version, category, subcategory, sub_subcategory
#   - has_light, has_dark, has_system (availability flags)
#   - preview_light, preview_dark, preview_system (preview image URLs)
#   - meta: { dependencies, tokens, tailwind } (extracted analysis)
#
# The script modifies files in-place. Source code is never written to output.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INPUT_DIR="${1:-}"

if [[ -z "$INPUT_DIR" ]]; then
  echo "Usage: $0 <input-dir>" >&2
  echo "  input-dir: Directory containing component NDJSON files" >&2
  exit 1
fi

if [[ ! -d "$INPUT_DIR" ]]; then
  echo "Error: Input directory does not exist: $INPUT_DIR" >&2
  exit 1
fi

# Temporary directory for processing
WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT

# Extract packages from import statements
# Input: component code (stdin)
# Output: JSON array of package names
extract_packages() {
  local result
  result=$(grep -oE "from ['\"](@[^'\"]+|[^@'\"]+)['\"]" 2>/dev/null \
    | sed -E "s/from ['\"]([^'\"]+)['\"].*/\1/" \
    | grep -v '^\.' \
    | sort -u \
    | jq -R . 2>/dev/null \
    | jq -s '.' 2>/dev/null) || true
  if [[ -z "$result" || "$result" == "null" ]]; then
    echo '[]'
  else
    echo "$result"
  fi
}

# Extract icon names from heroicons imports
# Input: component code (stdin)
# Output: JSON array of icon names
extract_icons() {
  local result
  # Handle multi-line imports by joining lines, then extracting icons
  result=$(tr '\n' ' ' \
    | grep -oE "import[[:space:]]*\{[^}]+\}[[:space:]]*from[[:space:]]*['\"]@heroicons/react" 2>/dev/null \
    | sed -E "s/import[[:space:]]*\{([^}]+)\}.*/\1/" \
    | tr ',' '\n' \
    | sed 's/^[[:space:]]*//' \
    | sed 's/[[:space:]]*$//' \
    | grep -v '^$' \
    | grep -E '^[A-Z].*Icon$' \
    | sort -u \
    | jq -R . 2>/dev/null \
    | jq -s '.' 2>/dev/null) || true
  if [[ -z "$result" || "$result" == "null" ]]; then
    echo '[]'
  else
    echo "$result"
  fi
}

# Extract Tailwind color tokens from className attributes
# Input: component code (stdin)
# Output: JSON array of color tokens
extract_colors() {
  local result
  result=$(grep -oE 'className="[^"]*"' 2>/dev/null \
    | sed 's/className="//g; s/"//g' \
    | tr ' ' '\n' \
    | grep -E '^(bg|text|border|ring|outline|shadow|from|to|via|divide|accent|caret|fill|stroke|decoration|placeholder)-[a-z]+-[0-9]+' 2>/dev/null \
    | sed -E 's/.*-(([a-z]+-)?[a-z]+-[0-9]+).*/\1/' \
    | sort -u \
    | jq -R . 2>/dev/null \
    | jq -s '.' 2>/dev/null) || true
  if [[ -z "$result" || "$result" == "null" ]]; then
    echo '[]'
  else
    echo "$result"
  fi
}

# Extract Tailwind spacing tokens
# Input: component code (stdin)
# Output: JSON array of spacing tokens
extract_spacing() {
  local result
  result=$(grep -oE 'className="[^"]*"' 2>/dev/null \
    | sed 's/className="//g; s/"//g' \
    | tr ' ' '\n' \
    | grep -E '^(p|px|py|pt|pr|pb|pl|m|mx|my|mt|mr|mb|ml|gap|gap-x|gap-y|space-x|space-y|inset|top|right|bottom|left)-[0-9]' 2>/dev/null \
    | sort -u \
    | jq -R . 2>/dev/null \
    | jq -s '.' 2>/dev/null) || true
  if [[ -z "$result" || "$result" == "null" ]]; then
    echo '[]'
  else
    echo "$result"
  fi
}

# Extract Tailwind typography tokens
# Input: component code (stdin)
# Output: JSON array of typography tokens
extract_typography() {
  local result
  result=$(grep -oE 'className="[^"]*"' 2>/dev/null \
    | sed 's/className="//g; s/"//g' \
    | tr ' ' '\n' \
    | grep -E '^(text-(xs|sm|base|lg|xl|2xl|3xl|4xl|5xl|6xl|7xl|8xl|9xl)|font-(thin|extralight|light|normal|medium|semibold|bold|extrabold|black)|tracking|leading|truncate|sr-only)' 2>/dev/null \
    | sort -u \
    | jq -R . 2>/dev/null \
    | jq -s '.' 2>/dev/null) || true
  if [[ -z "$result" || "$result" == "null" ]]; then
    echo '[]'
  else
    echo "$result"
  fi
}

# Extract v4-only Tailwind features
# Input: component code (stdin)
# Output: JSON array of v4-only feature prefixes
extract_v4_features() {
  local result
  result=$(grep -oE 'className="[^"]*"' 2>/dev/null \
    | sed 's/className="//g; s/"//g' \
    | tr ' ' '\n' \
    | grep -oE '^(data-[a-z-]+:|has-\[[^\]]+\]|inset-ring|inset-shadow|size-)' 2>/dev/null \
    | sed 's/:$//' \
    | sort -u \
    | jq -R . 2>/dev/null \
    | jq -s '.' 2>/dev/null) || true
  if [[ -z "$result" || "$result" == "null" ]]; then
    echo '[]'
  else
    echo "$result"
  fi
}

# Process a single component JSON object
# Input: JSON object (stdin)
# Output: JSON object with meta field added and source code STRIPPED
#
# The output contains:
#   - Component identifiers (id, uuid, name, category, etc.)
#   - Availability flags (has_light, has_dark, has_system)
#   - Preview URLs (preview_light, preview_dark, preview_system)
#   - Extracted metadata (dependencies, tokens, tailwind features)
#
# Source code is NOT included - fetch on-demand via authenticated session.
process_component() {
  local json
  json=$(cat)

  # Extract code from light mode (primary source for metadata extraction)
  local code
  code=$(echo "$json" | jq -r '.light.code // .dark.code // .system.code // ""')

  # Extract metadata from code (or use empty defaults)
  local packages icons colors spacing typography v4_features v3_compatible
  if [[ -z "$code" || "$code" == "null" ]]; then
    packages='[]'
    icons='[]'
    colors='[]'
    spacing='[]'
    typography='[]'
    v4_features='[]'
    v3_compatible="true"
  else
    packages=$(echo "$code" | extract_packages 2>/dev/null || echo '[]')
    icons=$(echo "$code" | extract_icons 2>/dev/null || echo '[]')
    colors=$(echo "$code" | extract_colors 2>/dev/null || echo '[]')
    spacing=$(echo "$code" | extract_spacing 2>/dev/null || echo '[]')
    typography=$(echo "$code" | extract_typography 2>/dev/null || echo '[]')
    v4_features=$(echo "$code" | extract_v4_features 2>/dev/null || echo '[]')

    # Determine v3 compatibility (no v4-only features)
    v3_compatible="true"
    if [[ "$v4_features" != "[]" ]]; then
      v3_compatible="false"
    fi
  fi

  # Build metadata-only output: strip code, keep availability flags + previews
  echo "$json" | jq -c --argjson packages "$packages" \
                       --argjson icons "$icons" \
                       --argjson colors "$colors" \
                       --argjson spacing "$spacing" \
                       --argjson typography "$typography" \
                       --argjson v4_only "$v4_features" \
                       --argjson v3_compat "$v3_compatible" \
    '{
      id,
      uuid,
      name,
      version,
      category,
      subcategory,
      sub_subcategory,
      has_light: (.light != null),
      has_dark: (.dark != null),
      has_system: (.system != null),
      preview_light: .light.preview,
      preview_dark: .dark.preview,
      preview_system: .system.preview,
      meta: {
        dependencies: {
          packages: $packages,
          icons: $icons
        },
        tokens: {
          colors: $colors,
          spacing: $spacing,
          typography: $typography
        },
        tailwind: {
          v4_only: $v4_only,
          v3_compatible: $v3_compat
        }
      }
    } | with_entries(select(.value != null))'
}

# Process all NDJSON files in the input directory
process_all_files() {
  local input_dir="$1"
  local files
  files=$(find "$input_dir" -name '*-v*.ndjson' -type f 2>/dev/null)

  if [[ -z "$files" ]]; then
    echo "Warning: No NDJSON files found in $input_dir" >&2
    return 0
  fi

  local total_files
  total_files=$(echo "$files" | wc -l | tr -d ' ')
  echo "Processing $total_files NDJSON files..."

  local file_count=0
  for file in $files; do
    file_count=$((file_count + 1))
    local filename
    filename=$(basename "$file")
    echo "[$file_count/$total_files] Processing $filename..."

    local output_file="$WORK_DIR/$filename"
    local line_count=0
    local total_lines
    total_lines=$(wc -l < "$file" | tr -d ' ')

    # Process each line
    while IFS= read -r line || [[ -n "$line" ]]; do
      line_count=$((line_count + 1))
      if [[ $((line_count % 50)) -eq 0 ]]; then
        echo "  Processing component $line_count/$total_lines..."
      fi
      echo "$line" | process_component >> "$output_file"
    done < "$file"

    # Replace original file
    mv "$output_file" "$file"
    echo "  Completed: $line_count components processed"
  done

  echo "Done! Processed $file_count files."
}

# Main
echo "=== Metadata Extraction ==="
echo "Input directory: $INPUT_DIR"
echo ""

process_all_files "$INPUT_DIR"

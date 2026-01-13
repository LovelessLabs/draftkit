#!/usr/bin/env bash
# Collect the Inertia Version
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CACHE_DIR="${CACHE_DIR:-$PROJECT_ROOT/cache}"
COOKIE_FILE="$CACHE_DIR/twp-cookies.txt"

if [[ -f "$CACHE_DIR/data-page.json" ]]; then
  jq -r '.version' "$CACHE_DIR/data-page.json"
else
  url='https://tailwindcss.com/plus'

  curl -sSL -b "$COOKIE_FILE" "$url" \
  | tr '\n' ' ' \
  | awk 'match($0,/data-page="[^"]+"/){ print substr($0,RSTART+11,RLENGTH-12); exit }' \
  | sed -e 's/&quot;/"/g' \
        -e 's/&apos;/'\''/g' \
        -e 's/&#039;/'\''/g' \
        -e 's/&#x27;/'\''/g' \
        -e 's/&#x2F;/\//g' \
        -e 's/&amp;/\&/g' \
        -e 's/&lt;/</g' \
        -e 's/&gt;/>/g' \
  | jq > "$CACHE_DIR/data-page.json"
  jq -r '.version' "$CACHE_DIR/data-page.json"
fi

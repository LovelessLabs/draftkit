#!/usr/bin/env bash
# TailwindCSS Plus authentication via curl
# Handles Laravel CSRF token + cookie-based session
#
# Credential sources (checked in order):
#   1. Environment variables: TWP_EMAIL, TWP_PASSWORD
#   2. 1Password CLI (op): fetches from item with website https://tailwindcss.com/plus
#   3. Credentials file: twp-credentials.json ({"email": "...", "password": "..."})

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CACHE_DIR="${CACHE_DIR:-$PROJECT_ROOT/cache}"
CREDS_FILE="${CREDS_FILE:-$PROJECT_ROOT/twp-credentials.json}"
COOKIE_FILE="${1:-$CACHE_DIR/twp-cookies.txt}"

# Ensure cache directory exists
mkdir -p "$CACHE_DIR"

# Convert absolute path to relative (./path)
relpath() {
  echo ".${1#$PROJECT_ROOT}"
}

# --- Credential Resolution ---

get_credentials() {
    # Source 1: Environment variables
    if [[ -n "$TWP_EMAIL" && -n "$TWP_PASSWORD" ]]; then
        echo "Using credentials from environment variables" >&2
        EMAIL="$TWP_EMAIL"
        PASSWORD="$TWP_PASSWORD"
        return 0
    fi

    # Source 2: 1Password CLI
    if command -v op &> /dev/null; then
        echo "Trying 1Password CLI..." >&2

        # Check if signed in
        if op account get &> /dev/null; then
            # Find item by website URL
            ITEM=$(op item list --format=json 2>/dev/null | \
                   jq -r '.[] | select(.urls[]?.href == "https://tailwindcss.com/plus") | .id' | head -1)

            if [[ -n "$ITEM" ]]; then
                EMAIL=$(op item get "$ITEM" --fields username 2>/dev/null || true)
                PASSWORD=$(op item get "$ITEM" --fields password --reveal 2>/dev/null || true)

                if [[ -n "$EMAIL" && -n "$PASSWORD" ]]; then
                    echo "Using credentials from 1Password" >&2
                    return 0
                fi
            fi
        fi
        echo "1Password: not signed in or no matching item found" >&2
    fi

    # Source 3: Credentials file
    if [[ -f "$CREDS_FILE" ]]; then
        echo "Using credentials from $CREDS_FILE" >&2
        EMAIL=$(jq -r '.email' "$CREDS_FILE")
        PASSWORD=$(jq -r '.password' "$CREDS_FILE")

        if [[ -n "$EMAIL" && "$EMAIL" != "null" && -n "$PASSWORD" && "$PASSWORD" != "null" ]]; then
            return 0
        fi
    fi

    return 1
}

# --- Main ---

if ! get_credentials; then
    echo "Error: No credentials found. Set TWP_EMAIL/TWP_PASSWORD, sign into 1Password, or create $CREDS_FILE" >&2
    exit 1
fi

urldecode() {
  # Convert %XX sequences to characters
  # Note: bash ${s//%/\\x} doesn't work because % is special in parameter expansion
  local decoded
  decoded=$(printf '%s' "$1" | sed 's/%/\\x/g')
  printf '%b' "$decoded"
}

get_xsrf() {
  raw=$(awk -F'\t' '$6=="XSRF-TOKEN"{print $7; exit}' "$COOKIE_FILE")
  [[ -n "$raw" ]] || { echo "get_xsrf: XSRF-TOKEN not found in $COOKIE_FILE" >&2; exit 1; }

  urldecode "$raw"
}

rm -f "$COOKIE_FILE"
echo "Step 1: Getting CSRF token and session cookies..."
curl -sS -c "$COOKIE_FILE" -o /dev/null 'https://tailwindcss.com/plus/login'

# Extract and URL-decode the XSRF token
XSRF_TOKEN=$(get_xsrf)
if [[ -z "$XSRF_TOKEN" ]]; then
    echo "Error: Could not extract XSRF token" >&2
    exit 1
fi

echo "Step 2: Logging in as $EMAIL..."
RESPONSE=$(curl -sS -w "\n%{http_code}" \
    -b "$COOKIE_FILE" -c "$COOKIE_FILE" \
    -H "X-XSRF-TOKEN: $XSRF_TOKEN" \
    -H "Origin: https://tailwindcss.com" \
    -H "Referer: https://tailwindcss.com/plus/login" \
    -d "email=$EMAIL&password=$PASSWORD" \
    'https://tailwindcss.com/plus/login')

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

# 302 = redirect after successful login
if [[ "$HTTP_CODE" == "302" || "$HTTP_CODE" == "200" ]]; then
    echo "Login successful! Cookies saved to: $(relpath "$COOKIE_FILE")"
else
    echo "Login failed with HTTP $HTTP_CODE" >&2
    exit 1
fi

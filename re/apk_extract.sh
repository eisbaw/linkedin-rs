#!/usr/bin/env bash
# apk_extract.sh — Extract APK or XAPK into extracted/
#
# Usage: bash re/apk_extract.sh <file.apk|file.xapk>
#
# APK:  Plain ZIP containing classes.dex, AndroidManifest.xml, etc.
#       Extracted directly into extracted/
#
# XAPK: ZIP bundle containing multiple APK files + manifest.json.
#       Outer ZIP extracted to a temp dir, then each inner .apk
#       is extracted into extracted/<apk-basename>/

set -euo pipefail

die() { echo "ERROR: $*" >&2; exit 1; }

[[ $# -eq 1 ]] || die "Usage: $0 <file.apk|file.xapk>"

INPUT_FILE="$1"
[[ -f "$INPUT_FILE" ]] || die "File not found: $INPUT_FILE"

# Resolve to absolute path
INPUT_FILE="$(realpath "$INPUT_FILE")"
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
EXTRACT_DIR="$SCRIPT_DIR/extracted"

# Detect format by inspecting ZIP contents.
# XAPK bundles contain inner .apk files and usually a manifest.json.
# Plain APK files contain AndroidManifest.xml at the top level.
detect_format() {
    local file="$1"
    local listing
    listing="$(unzip -l "$file" 2>&1)" || die "Failed to list ZIP contents of $file"

    # XAPK bundles contain inner .apk entries (not the Archive: header line).
    # Match lines that look like ZIP entries: leading whitespace + size + date + name.apk
    if grep -qP '^\s+\d+\s+.*\.apk$' <<< "$listing"; then
        echo "xapk"
    elif grep -q 'AndroidManifest.xml' <<< "$listing"; then
        echo "apk"
    else
        die "Cannot detect format — no inner .apk files and no AndroidManifest.xml found"
    fi
}

extract_apk() {
    local apk_file="$1"
    local target_dir="$2"

    echo "Extracting APK: $(basename "$apk_file") -> $target_dir"
    mkdir -p "$target_dir"
    unzip -o -q "$apk_file" -d "$target_dir"
    echo "  Done. $(find "$target_dir" -type f | wc -l) files extracted."
}

extract_xapk() {
    local xapk_file="$1"

    echo "Detected XAPK bundle. Extracting outer ZIP..."
    local tmp_dir
    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "$tmp_dir"' EXIT

    unzip -o -q "$xapk_file" -d "$tmp_dir"

    # List inner APKs
    local inner_apks=()
    while IFS= read -r -d '' f; do
        inner_apks+=("$f")
    done < <(find "$tmp_dir" -name '*.apk' -print0)

    if [[ ${#inner_apks[@]} -eq 0 ]]; then
        die "XAPK bundle contains no inner .apk files"
    fi

    echo "Found ${#inner_apks[@]} inner APK(s)."

    # Copy non-APK files (manifest.json, icons, etc.) to extracted root
    echo "Copying bundle metadata..."
    find "$tmp_dir" -maxdepth 1 -type f ! -name '*.apk' -exec cp {} "$EXTRACT_DIR/" \;

    # Extract each inner APK into its own subdirectory
    for apk in "${inner_apks[@]}"; do
        local basename
        basename="$(basename "$apk" .apk)"
        extract_apk "$apk" "$EXTRACT_DIR/$basename"
    done
}

# --- Main ---

FORMAT="$(detect_format "$INPUT_FILE")"
echo "Input:  $INPUT_FILE"
echo "Format: $FORMAT"
echo "Output: $EXTRACT_DIR"
echo ""

# Clean previous extraction
if [[ -d "$EXTRACT_DIR" ]]; then
    echo "Cleaning previous extraction..."
    rm -rf "$EXTRACT_DIR"
fi
mkdir -p "$EXTRACT_DIR"

case "$FORMAT" in
    apk)
        extract_apk "$INPUT_FILE" "$EXTRACT_DIR"
        ;;
    xapk)
        extract_xapk "$INPUT_FILE"
        ;;
esac

echo ""
echo "Extraction complete."
echo "Total files: $(find "$EXTRACT_DIR" -type f | wc -l)"
echo "Total size:  $(du -sh "$EXTRACT_DIR" | cut -f1)"

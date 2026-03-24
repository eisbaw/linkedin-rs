#!/usr/bin/env bash
# LinkedIn-specific PII scanner for tracked git files.
# Scans for LinkedIn profile URNs, member IDs, real names in docs,
# hardcoded session tokens, and other LinkedIn-specific PII patterns.
#
# Usage: ./scripts/linkedin-pii-scan.sh [--diff]
#   No args: scans all tracked files
#   --diff:  scans only staged + unstaged changes (for pre-commit)
#
# Exit code: 0 = clean, 1 = findings

set -euo pipefail

FINDINGS=0
RED='\033[0;31m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
NC='\033[0m'

# Directories to skip (gitignored content)
SKIP_DIRS="secrets|extracted|decompiled|analysis|reports|har|target"

scan_content() {
    local label="$1"
    local input_mode="$2"  # "files" or "diff"

    local rg_base=(rg --no-filename -n --color never)
    if [ "$input_mode" = "diff" ]; then
        rg_base=(rg --no-filename -n --color never)
    fi

    local rg_target=()
    if [ "$input_mode" = "files" ]; then
        # Get tracked files, excluding skip dirs and binary files
        mapfile -t rg_target < <(git ls-files --cached | grep -vE "^($SKIP_DIRS)/" | grep -vE '\.(apk|xapk|png|jpg|gif|ico|woff|ttf|lock)$')
        if [ ${#rg_target[@]} -eq 0 ]; then
            return
        fi
    fi

    # --- Pattern 1: LinkedIn profile URNs (fsd_profile, fs_miniProfile, member) ---
    # Matches actual encoded profile IDs like ACoAABxxxxxx...
    # Excludes placeholder patterns and code that constructs URNs dynamically
    local urn_pattern='urn:li:(fsd_profile|fs_miniProfile|member):[A-Za-z0-9_-]{10,}'
    local hits
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never "$urn_pattern" "${rg_target[@]}" 2>/dev/null | grep -vE '(\.rs:|Cargo\.).*urn:li:(fsd_profile|fs_miniProfile):\{' | grep -vE 'replace\(|format!\(|urn:li:fsd_profile:\.\.\.' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never "$urn_pattern" 2>/dev/null | grep -vE 'replace\(|format!\(' || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${RED}HIGH${NC}  LinkedIn Profile URN"
        echo "$hits" | head -20 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 2: LinkedIn member numeric IDs ---
    # Bare numeric member IDs (6-10 digits) near LinkedIn context
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never 'member[_:]?\s*:?\s*[0-9]{6,10}' "${rg_target[@]}" 2>/dev/null | grep -vE '(\.rs:|\.toml:).*member' | grep -ivE 'example|placeholder|test' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never 'member[_:]?\s*:?\s*[0-9]{6,10}' 2>/dev/null | grep -ivE 'example|placeholder' || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${RED}HIGH${NC}  LinkedIn Member Numeric ID"
        echo "$hits" | head -10 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 3: li_at session cookie values ---
    # AQE... pattern with 200+ chars
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never 'AQE[A-Za-z0-9_-]{50,}' "${rg_target[@]}" 2>/dev/null | grep -vE '\.rs:.*"AQE' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never 'AQE[A-Za-z0-9_-]{50,}' 2>/dev/null || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${RED}CRITICAL${NC}  li_at Session Cookie Value"
        echo "$hits" | head -5 | while IFS= read -r line; do
            echo "  ${line:0:80}..."
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 4: JSESSIONID values ---
    # ajax: followed by 16-20 digit number
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never 'ajax:[0-9]{16,20}' "${rg_target[@]}" 2>/dev/null | grep -vE '\.rs:.*ajax:\{|generate_jsessionid|format!|ajax:\{19' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never 'ajax:[0-9]{16,20}' 2>/dev/null | grep -vE 'generate_jsessionid|format!' || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${RED}HIGH${NC}  JSESSIONID Value"
        echo "$hits" | head -5 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 5: LinkedIn conversation/message URNs with encoded IDs ---
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never 'urn:li:msg_(conversation|message|messagingParticipant):[^\s"]{20,}' "${rg_target[@]}" 2>/dev/null | grep -vE '\.rs:' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never 'urn:li:msg_(conversation|message):[^\s"]{20,}' 2>/dev/null || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${YELLOW}MEDIUM${NC}  LinkedIn Message/Conversation URN"
        echo "$hits" | head -10 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 6: Real person names near LinkedIn context ---
    # Look for firstName/lastName JSON patterns with actual values (not code)
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never '"(firstName|lastName)"\s*:\s*"[A-Z][a-z]+' "${rg_target[@]}" 2>/dev/null | grep -vE '\.rs:|test|example|placeholder|mock|dummy' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never '"(firstName|lastName)"\s*:\s*"[A-Z][a-z]+' 2>/dev/null | grep -vE 'test|example' || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${RED}HIGH${NC}  Real Person Name (firstName/lastName in JSON)"
        echo "$hits" | head -10 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 7: publicIdentifier / vanity URL slugs with real values ---
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never '"publicIdentifier"\s*:\s*"[a-z][-a-z0-9]{2,}' "${rg_target[@]}" 2>/dev/null | grep -vE '\.rs:|test|example|placeholder' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never '"publicIdentifier"\s*:\s*"[a-z][-a-z0-9]{2,}' 2>/dev/null || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${RED}HIGH${NC}  LinkedIn Public Identifier (vanity URL)"
        echo "$hits" | head -10 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi

    # --- Pattern 8: Occupation/headline with real company names ---
    if [ "$input_mode" = "files" ]; then
        hits=$(rg -n --color never '"occupation"\s*:\s*"[A-Z][^"]{10,}' "${rg_target[@]}" 2>/dev/null | grep -vE '\.rs:|test|example|placeholder' || true)
    else
        hits=$(echo "$DIFF_CONTENT" | grep -E '^\+' | rg --color never '"occupation"\s*:\s*"[A-Z][^"]{10,}' 2>/dev/null || true)
    fi
    if [ -n "$hits" ]; then
        echo -e "${YELLOW}MEDIUM${NC}  Real Occupation/Headline"
        echo "$hits" | head -10 | while IFS= read -r line; do
            echo "  $line"
        done
        FINDINGS=$((FINDINGS + 1))
    fi
}

# Main
echo "=== LinkedIn PII Scan ==="
echo ""

if [ "${1:-}" = "--diff" ]; then
    DIFF_CONTENT=$(git diff --cached 2>/dev/null; git diff 2>/dev/null)
    if [ -z "$DIFF_CONTENT" ]; then
        echo -e "${GREEN}PASS${NC} -- no staged or unstaged changes to scan."
        exit 0
    fi
    scan_content "diff" "diff"
else
    scan_content "tracked files" "files"
fi

echo ""
if [ "$FINDINGS" -gt 0 ]; then
    echo -e "${RED}FAIL${NC} -- $FINDINGS pattern(s) matched. Review findings above."
    exit 1
else
    echo -e "${GREEN}PASS${NC} -- no LinkedIn-specific PII found in tracked files."
    exit 0
fi

#!/usr/bin/env bash
# Report Wolfram-Language name coverage against docs/wl-list.txt.
# List format: one "Name<TAB>category" per line (category optional).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

LIST="${1:-docs/wl-list.txt}"
[[ -f "$LIST" ]] || { echo "missing $LIST"; exit 1; }

have="$(mktemp)"
{ grep -rhoE '"[A-Za-z$][A-Za-z0-9$]*"' syma/src syma/SystemFiles 2>/dev/null | tr -d '"'; } | sort -u > "$have"

total=$(cut -f1 "$LIST" | sort -u | wc -l)
miss=$(comm -23 <(cut -f1 "$LIST" | sort -u) "$have" | wc -l)
echo "coverage: $((total-miss))/$total present, MISSING=$miss"
echo "--- missing (name<TAB>category) ---"
awk -F'\t' 'NR==FNR{miss[$1]=1;next} ($1 in miss){print}' \
  <(comm -23 <(cut -f1 "$LIST" | sort -u) "$have") "$LIST" | head -80
rm -f "$have"

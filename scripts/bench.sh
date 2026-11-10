#!/usr/bin/env bash
# Micro-benchmarks for hot symbolic paths; prints wall time per case.
# Used to detect performance regressions between rounds.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="$HOME/.cargo/bin:$PATH"
: "${CARGO_TARGET_DIR:=/tmp/opencode/t-bench}"
export CARGO_TARGET_DIR

cargo build -q -p syma --locked
BIN="$CARGO_TARGET_DIR/debug/syma"

cases=(
  'Simplify[Cos[x]^2 - Sin[x]^2]'
  'Simplify[(x^2 - 1)/(x - 1)]'
  'Integrate[1/(1 + x^2), x]'
  'Integrate[x^2 Exp[x], x]'
  'D[Sin[x^2]^3, x]'
  'Limit[Sin[x]/x, x -> 0]'
  'Sum[1/k^2, {k, 1, Infinity}]'
  'Series[Log[1 + x], {x, 0, 6}]'
  'Solve[x^3 - 6 x^2 + 11 x - 6 == 0, x]'
  'Expand[(x + y + z)^8]'
)

for c in "${cases[@]}"; do
  t0=$(date +%s.%N)
  "$BIN" -e "$c" >/dev/null 2>&1 || true
  t1=$(date +%s.%N)
  printf '%8.3fs  %s\n' "$(echo "$t1 - $t0" | bc)" "$c"
done

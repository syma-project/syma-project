#!/usr/bin/env bash
# 30-minute progress checkpoint: metrics + fast gate + forward-plan refresh.
# Installed via crontab (see scripts/install_cron.sh). Safe to run by hand.
set -uo pipefail

REPO=/data/projects/11620001/anketang/syma-project
cd "$REPO" || exit 1

export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin"
export RUST_MIN_STACK=134217728
export CARGO_TARGET_DIR=/tmp/opencode/t-cron

LOG="$REPO/docs/PROGRESS_LOG.md"
NEXT="$REPO/docs/NEXT_ACTIONS.md"
NOW="$(date '+%Y-%m-%d %H:%M:%S %Z')"

head_super="$(git rev-parse --short HEAD 2>/dev/null || echo -)"
head_syma="$(git -C syma rev-parse --short HEAD 2>/dev/null || echo -)"
dirty="$(git -C syma status --porcelain 2>/dev/null | wc -l)"
cats="$(grep -c '#\[test\]' syma/tests/wl_math.rs 2>/dev/null || echo 0)"
allows="$(grep -rhc 'allow(dead_code\|allow(clippy' syma/src 2>/dev/null | paste -sd+ | bc 2>/dev/null || echo -)"
libtests="$(grep -rho '#\[test\]' syma/src 2>/dev/null | wc -l)"

cov="$(bash scripts/coverage.sh 2>/dev/null | head -1 || echo 'coverage: n/a')"

# --- fast gate: fmt + clippy + corpus (incremental; timeouts to bound cost) ---
gate="?"
{ timeout 240 cargo fmt --all --check >/dev/null 2>&1; } || { echo ""; }
fmt_rc=0; cargo fmt --all --check >/dev/null 2>&1 || fmt_rc=1
clip_rc=0; timeout 900 cargo clippy --workspace --all-targets --locked -- -D warnings >/dev/null 2>&1 || clip_rc=1
corpus_rc=0; timeout 600 cargo test -p syma --locked --test wl_math >/dev/null 2>&1 || corpus_rc=1
if [[ $fmt_rc -eq 0 && $clip_rc -eq 0 && $corpus_rc -eq 0 ]]; then gate="GREEN"; else gate="RED(fmt=$fmt_rc clippy=$clip_rc corpus=$corpus_rc)"; fi

# --- append checkpoint ---
{
  echo ""
  echo "## $NOW"
  echo "- gate: **$gate**"
  echo "- HEAD: super \`$head_super\` / syma \`$head_syma\`; dirty(syma)=$dirty"
  echo "- corpus categories: $cats; src #[test]: $libtests; allow(dead_code|clippy) sites: $allows"
  echo "- $cov"
} >> "$LOG"

# --- refresh forward plan from coverage gaps ---
{
  echo "# NEXT_ACTIONS (auto-generated $NOW)"
  echo ""
  echo "由 scripts/progress_check.sh 依据 docs/wl-list.txt 缺口生成；按类别聚类，映射到建议文件。"
  echo "每次检查覆盖写入；人工可在下方“Manual notes”追加。"
  echo ""
  echo "## 缺口聚类（Top 40）"
  echo '```'
  bash scripts/coverage.sh 2>/dev/null | tail -n +3 | head -40
  echo '```'
  echo ""
  echo "## 建议顺序（按类别→文件）"
  echo "- graph → syma/src/builtins/graphtheory.rs"
  echo "- calculus/transforms → syma/src/builtins/{symbolic,symbolicmanip,calculusmore,transforms}.rs"
  echo "- algebra/solve → syma/src/builtins/{polyreduce,polynomial,symbolicmanip}.rs"
  echo "- special → syma/src/builtins/{specialfunctions,specialfuncmore}.rs"
  echo "- number theory/combinatorics → syma/src/builtins/{number_theory,combinatorics}.rs"
  echo "- linalg/tensor/sparse → syma/src/builtins/{linalg,linalgmore,tensor}.rs"
  echo "- stats/probability/ml → syma/src/builtins/{statistics,statsmore,fit,timeseries,mllite}.rs"
  echo "- io/notebook/streams → syma/src/builtins/io/*, {notebook,streams}.rs"
  echo "- runtime → syma/src/{eval,bytecode,jit,ffi}/*"
  echo ""
  echo "## Manual notes"
  echo "- (append here)"
} > "$NEXT"

echo "[progress_check] $NOW gate=$gate cats=$cats dirty=$dirty heads=$head_super/$head_syma :: $cov"

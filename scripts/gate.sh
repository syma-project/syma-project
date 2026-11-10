#!/usr/bin/env bash
# Syma per-round hard gate. Must be fully green before any round is committed.
#
# Usage:  scripts/gate.sh [--fast]
#   --fast : skip the (slow) full workspace test; still runs fmt + clippy + corpus.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export PATH="$HOME/.cargo/bin:$PATH"
export RUST_MIN_STACK=67108864
: "${CARGO_TARGET_DIR:=/tmp/opencode/t-gate}"
export CARGO_TARGET_DIR

FAST=0
[[ "${1:-}" == "--fast" ]] && FAST=1

step() { printf '\n\033[1;36m[gate] %s\033[0m\n' "$1"; }

step "rustfmt --check"
cargo fmt --all --check

step "clippy -D warnings (workspace, all targets)"
cargo clippy --workspace --all-targets --locked -- -D warnings

if [[ "$FAST" == "0" ]]; then
  step "cargo test --workspace"
  cargo test --locked --workspace
fi

step "WL conformance corpus (tests/wl_math.rs)"
cargo test -p syma --locked --test wl_math

printf '\n\033[1;32m[gate] OK — all checks passed\033[0m\n'

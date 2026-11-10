#!/usr/bin/env bash
# Wake the long-running planning session with a checkpoint prompt.
# Run by cron every 30 min (offset). Serialized by the caller via flock.
set -uo pipefail
REPO=/data/projects/11620001/anketang/syma-project
OC="$HOME/.opencode/bin/opencode"
MODEL="gserve/deepseek-v4.1-flash-uncensored"
SID="${SYMA_SESSION:-ses_e3c32ff97ffepmaolu4z4M7Oei}"
cd "$REPO" || exit 1
MSG="$(cat "$REPO/scripts/wake_msg.txt")"
exec "$OC" run --auto --model "$MODEL" --session "$SID" "$MSG"

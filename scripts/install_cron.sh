#!/usr/bin/env bash
# Install/refresh the 30-minute cron jobs:
#   (1) progress_check.sh  -> metrics + gate + NEXT_ACTIONS.md  (files only)
#   (2) wake.sh            -> enqueue a checkpoint prompt into the agent session
set -euo pipefail
REPO=/data/projects/11620001/anketang/syma-project
touch "$REPO/.progress-cron.log" "$REPO/.wake-cron.log"
L1="*/30 * * * * cd $REPO && $REPO/scripts/progress_check.sh >> $REPO/.progress-cron.log 2>&1"
L2="3,33 * * * * cd $REPO && flock -n /tmp/opencode/wake.lock timeout 1800 $REPO/scripts/wake.sh >> $REPO/.wake-cron.log 2>&1"
cur="$(crontab -l 2>/dev/null | grep -v 'progress_check.sh' | grep -v 'scripts/wake' || true)"
{ [[ -n "$cur" ]] && echo "$cur"; echo "$L1"; echo "$L2"; } | crontab -
echo "installed:"; crontab -l

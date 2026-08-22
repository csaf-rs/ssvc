#!/usr/bin/env bash
# Runs the full extraction/analysis pipeline end-to-end against a checkout
# of https://github.com/CERTCC/SSVC (e.g. this repo's `ssvc/` submodule).
#
# Usage: ./run_all.sh /path/to/SSVC/checkout [output_dir]
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="${1:?Usage: run_all.sh /path/to/SSVC/checkout [output_dir]}"
OUT_DIR="${2:-.}"
mkdir -p "$OUT_DIR"

echo "[0/3] Fetching GitHub release metadata..." >&2
"$SCRIPT_DIR/0_fetch_releases.sh" "$SCRIPT_DIR/releases_sorted.txt"

echo "[1/3] Extracting decision point version definitions from $REPO_DIR ..." >&2
python3 "$SCRIPT_DIR/1_extract_decision_points.py" "$REPO_DIR" \
  > "$OUT_DIR/dps_list.txt" 2> "$OUT_DIR/dps_list.err"

echo "[2/3] Tracing each version to its introducing commit + first release..." >&2
python3 "$SCRIPT_DIR/2_analyze_git_history.py" "$REPO_DIR" "$OUT_DIR/dps_list.txt" \
  > "$OUT_DIR/dps_analyzed.txt"

echo "[3/3] Building markdown summary table..." >&2
python3 "$SCRIPT_DIR/3_build_markdown_table.py" "$OUT_DIR/dps_analyzed.txt" \
  > "$OUT_DIR/table_body.md"

echo "Done. Results in $OUT_DIR/{dps_list.txt,dps_analyzed.txt,table_body.md}" >&2

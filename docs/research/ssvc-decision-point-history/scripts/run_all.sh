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

echo "[0/4] Fetching GitHub release metadata..." >&2
"$SCRIPT_DIR/0_fetch_releases.sh" "$SCRIPT_DIR/releases_sorted.txt"

echo "[1/4] Extracting decision point version definitions from $REPO_DIR ..." >&2
python3 "$SCRIPT_DIR/1_extract_decision_points.py" "$REPO_DIR" \
  > "$OUT_DIR/dps_list.txt" 2> "$OUT_DIR/dps_list.err"

echo "[2/4] Tracing each version to its introducing commit + first release..." >&2
python3 "$SCRIPT_DIR/2_analyze_git_history.py" "$REPO_DIR" "$OUT_DIR/dps_list.txt" \
  > "$OUT_DIR/dps_analyzed.txt"

echo "[3/4] Building markdown summary table..." >&2
python3 "$SCRIPT_DIR/3_build_markdown_table.py" "$OUT_DIR/dps_analyzed.txt" \
  > "$OUT_DIR/table_body.md"

echo "[4/4] Building structured namespace/key/version JSON..." >&2
python3 "$SCRIPT_DIR/4_build_structured_data.py" "$OUT_DIR/dps_analyzed.txt" \
  > "$OUT_DIR/decision-points-by-namespace-key-version.json"

echo "Done. Results in $OUT_DIR/{dps_list.txt,dps_analyzed.txt,table_body.md,decision-points-by-namespace-key-version.json}" >&2


#!/usr/bin/env bash
# Fetches published GitHub Releases (tag + published_at) for CERTCC/SSVC and
# writes them sorted-by-date to releases_sorted.txt, in the same format
# consumed by 2_analyze_git_history.py ("<tag>|<published_at ISO8601>").
#
# Usage: ./0_fetch_releases.sh [output_file]
set -euo pipefail
OUT="${1:-$(dirname "$0")/releases_sorted.txt}"

curl -s "https://api.github.com/repos/CERTCC/SSVC/releases?per_page=100" \
  | python3 -c '
import json, sys
data = json.load(sys.stdin)
if isinstance(data, dict):
    sys.exit(f"GitHub API error: {data}")
rows = [(r["tag_name"], r["published_at"]) for r in data]
rows.sort(key=lambda x: x[1])
for tag, date in rows:
    print(f"{tag}|{date}")
' > "$OUT"

echo "Wrote $(wc -l < "$OUT") releases to $OUT" >&2

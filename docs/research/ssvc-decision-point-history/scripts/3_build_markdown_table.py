import datetime, sys

# Usage: python3 3_build_markdown_table.py [dps_analyzed.txt]  (output of step 2)
INPUT_FILE = sys.argv[1] if len(sys.argv) > 1 else "dps_analyzed.txt"

rows = []
with open(INPUT_FILE) as f:
    for line in f:
        parts = line.rstrip("\n").split("|")
        namespace, path, varname, name, key, version, commit, commit_date, subject, tag, tag_date = parts
        rows.append(dict(namespace=namespace, path=path, varname=varname, name=name, key=key,
                          version=version, commit=commit, commit_date=commit_date, subject=subject,
                          tag=tag, tag_date=tag_date))

def fmt(d):
    if d in (None, "None"):
        return "?"
    return d[:10]

# group by namespace then file
from collections import defaultdict
by_ns = defaultdict(list)
for r in rows:
    by_ns[r["namespace"]].append(r)

ns_order = ["ssvc", "cvss", "cisa", "nist", "basic", "example"]

lines = []
for ns in ns_order:
    items = by_ns.get(ns, [])
    if not items:
        continue
    by_file = defaultdict(list)
    for r in items:
        by_file[r["path"]].append(r)
    lines.append(f"\n### `{ns}` namespace\n")
    lines.append("| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |")
    lines.append("|---|---|---|---|---|")
    for path in sorted(by_file):
        recs = by_file[path]
        # sort by version tuple
        def vkey(r):
            return tuple(int(x) for x in r["version"].split("."))
        recs.sort(key=vkey)
        for r in recs:
            lines.append(f"| {r['name'].strip()} ({r['key']}) | {r['version']} | {fmt(r['commit_date'])} | {r['tag'] or '?'} | {fmt(r['tag_date'])} |")

print("\n".join(lines))

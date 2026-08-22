import os, subprocess, sys

# Usage: python3 2_analyze_git_history.py <repo-dir> <dps_list.txt>
#
# For every decision-point version emitted by step 1, finds the oldest
# commit that introduced its exact `version="x.y.z"` literal in its source
# file, and the earliest published GitHub Release containing that commit.
#
# IMPORTANT git quirk: `git log --follow ... --reverse` silently returns
# NOTHING when combined with `-G`/`-S` pickaxe search in this git version
# (2.50.1) — `--follow` (needed because several decision-point files were
# renamed/moved across directories over time, e.g.
# decision_points/exploitation.py -> decision_points/ssvc_/exploitation.py
# -> decision_points/ssvc/exploitation.py) is NOT compatible with
# `--reverse` here. Workaround: omit `--reverse`, fetch commits newest-first,
# and take the LAST line ourselves as the oldest/introducing commit.
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_DIR = sys.argv[1] if len(sys.argv) > 1 else "."
RELEASES_FILE = os.path.join(SCRIPT_DIR, "releases_sorted.txt")
DPS_LIST_FILE = sys.argv[2] if len(sys.argv) > 2 else "dps_list.txt"

releases = []
with open(RELEASES_FILE) as f:
    for line in f:
        tag, date = line.strip().split("|")
        releases.append((tag, date))
release_tags = set(t for t, _ in releases)
release_date = dict(releases)


def run(cmd):
    return subprocess.run(cmd, cwd=REPO_DIR, capture_output=True, text=True).stdout


results = []
with open(DPS_LIST_FILE) as f:
    for line in f:
        line = line.rstrip("\n")
        if not line or line.startswith("TOTAL"):
            continue
        namespace, path, varname, name, key, version = line.split("|")
        search = f'version="{version}"'
        out = run(["git", "log", "--follow", "-G", search,
                    "--format=%H|%aI|%s", "--", path])
        lines = [l for l in out.strip().split("\n") if l]
        if not lines:
            results.append((namespace, path, varname, name, key, version,
                             None, None, None, None, None))
            continue
        commit, commit_date, subject = lines[-1].split("|", 2)
        tags_out = run(["git", "tag", "--contains", commit])
        tags = [t for t in tags_out.strip().split("\n") if t in release_tags]
        if tags:
            tags_sorted = sorted(tags, key=lambda t: release_date[t])
            earliest_tag = tags_sorted[0]
            earliest_date = release_date[earliest_tag]
        else:
            earliest_tag, earliest_date = None, None
        results.append((namespace, path, varname, name, key, version,
                         commit, commit_date, subject, earliest_tag, earliest_date))

for r in results:
    print("|".join(str(x) for x in r))

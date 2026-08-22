import os, subprocess, sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_DIR = sys.argv[1] if len(sys.argv) > 1 else "."  # path to CERTCC/SSVC checkout
RELEASES_FILE = os.path.join(SCRIPT_DIR, "releases_sorted.txt")
DPS_LIST_FILE = sys.argv[2] if len(sys.argv) > 2 else "dps_list.txt"  # output of step 1

releases = []
with open(RELEASES_FILE) as f:
    for line in f:
        tag, date = line.strip().split("|")
        releases.append((tag, date))
release_tags = set(t for t,_ in releases)
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
        # get oldest commit introducing this exact version string in this file
        out = run(["git", "log", "--diff-filter=A,M", "-G", search,
                    "--format=%H|%aI", "--reverse", "--", path])
        lines = [l for l in out.strip().split("\n") if l]
        if not lines:
            # fallback: without --diff-filter
            out = run(["git", "log", "-G", search, "--format=%H|%aI", "--reverse", "--", path])
            lines = [l for l in out.strip().split("\n") if l]
        if not lines:
            results.append((namespace, path, varname, name, key, version, None, None, None))
            continue
        commit, commit_date = lines[0].split("|")
        # find release tags containing this commit
        tags_out = run(["git", "tag", "--contains", commit])
        tags = [t for t in tags_out.strip().split("\n") if t in release_tags]
        if tags:
            # pick earliest by published date
            tags_sorted = sorted(tags, key=lambda t: release_date[t])
            earliest_tag = tags_sorted[0]
            earliest_date = release_date[earliest_tag]
        else:
            earliest_tag, earliest_date = None, None
        results.append((namespace, path, varname, name, key, version, commit_date, earliest_tag, earliest_date))

for r in results:
    print("|".join(str(x) for x in r))

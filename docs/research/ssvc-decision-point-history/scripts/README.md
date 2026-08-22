# SSVC Decision Point Version History — scripts

Ad-hoc scripts used to produce `../README.md`.
Not intended as production code — a starting point for the follow-up
"pipeline" task.

## Pipeline

1. `0_fetch_releases.sh` — pulls published GitHub Releases (tag +
   `published_at`) for CERTCC/SSVC via the REST API, sorted by date, into
   `releases_sorted.txt`. This is more reliable than tag-creation dates,
   since several early tags (`v1.0`, `v1.1`, `v2.0`) were only turned into
   GitHub Releases retroactively in 2023.
2. `1_extract_decision_points.py <repo-dir>` — walks
   `src/ssvc/decision_points/` in a CERTCC/SSVC checkout, AST-parses every
   `*.py` file (skipping `__init__.py`/`base.py`/`helpers.py`/
   `_not_defined.py`), and finds every top-level assignment of the form
   `NAME = SomeDecisionPoint(..., key=..., version=..., name=...)`. Emits
   `namespace|path|varname|name|key|version` lines.
3. `2_analyze_git_history.py <repo-dir> <dps_list.txt>` — for each extracted
   decision point version, runs
   `git log -G'version="x.y.z"' -- <file>` (note: `-G`, not `-S` — `-S`
   pickaxe combined with `--follow` silently returned nothing in testing,
   `-G` regex pickaxe worked reliably) to find the oldest commit that
   introduced that exact version string in that file, then finds the
   earliest published release tag containing that commit. Emits the same
   columns plus `commit_date|first_release_tag|first_release_date`.
4. `3_build_markdown_table.py <dps_analyzed.txt>` — groups the results by
   namespace/file, sorts versions ascending, and renders a markdown table.

Run everything in one go:

```sh
./run_all.sh /path/to/SSVC/checkout ./out
```

(In this session, `/path/to/SSVC/checkout` was this repo's `ssvc/` git
submodule, which is a full clone of CERTCC/SSVC with all tags fetched.)

## Known quirks / gotchas

- `git log --follow -G<pattern>` returned **empty output** in several cases
  even though `-G<pattern>` alone (without `--follow`) found the commit.
  The scripts therefore do **not** use `--follow`; this is fine as long as
  the decision-point files haven't been renamed (verified spot-checks
  showed they haven't).
- `git tag --contains <commit>` returns *all* tags containing the commit,
  including many non-release CI/dev tags in the CERTCC/SSVC repo (e.g.
  `2025.09.1223`, `fix-834-attempt-1`, `testingtag`, `old_json`). Only tags
  present in `releases_sorted.txt` (i.e. actual GitHub Releases) are
  considered.
- Some decision points appear to change `name`/`key` across versions while
  keeping the same conceptual identity (e.g. CVSS `E` key: "Exploitability"
  → "Exploit Code Maturity" → "Exploit Maturity"). The extraction keeps
  these as separate rows grouped by file, not by a stable decision-point
  identity — the follow-up pipeline will need to define one.

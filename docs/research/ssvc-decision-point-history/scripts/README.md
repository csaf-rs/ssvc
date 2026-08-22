# SSVC Decision Point Version History — scripts

Ad-hoc scripts used to produce `../README.md` and
`../decision-points-by-namespace-key-version.json`. Not intended as
production code — a starting point for the follow-up "pipeline" task.

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
   decision point version, runs `git log --follow -G'version="x.y.z"' --
   <file>` to find the oldest commit that introduced that exact version
   string in that file (tracking renames — see gotcha below), then finds
   the earliest published release tag containing that commit. Emits the
   same columns plus
   `commit|commit_date|commit_subject|first_release_tag|first_release_date`.
4. `3_build_markdown_table.py <dps_analyzed.txt>` — groups the results by
   namespace/file, sorts versions ascending, and renders a markdown table.
5. `4_build_structured_data.py <dps_analyzed.txt>` — groups the same results
   into `namespace -> key -> version` JSON, and attaches a `retroactive`
   classification (see `../README.md`) per introducing commit subject.

Run everything in one go:

```sh
./run_all.sh /path/to/SSVC/checkout ./out
```

(In this session, `/path/to/SSVC/checkout` was this repo's `ssvc/` git
submodule, which is a full clone of CERTCC/SSVC with all tags fetched.)

## Known quirks / gotchas

- **`git log --follow -G<pattern> --reverse` silently returns NOTHING** in
  this git version (2.50.1) whenever the file being searched was
  renamed/moved at some point — which applies to almost every file under
  `src/ssvc/decision_points/` (the package was reorganized multiple times,
  e.g. `decision_points/exploitation.py` →
  `decision_points/ssvc_/exploitation.py` →
  `decision_points/ssvc/exploitation.py`). An earlier version of this
  script omitted `--follow` to work around a *different* symptom of the
  same bug, which silently produced results based only on the file's
  history *after* its last rename — systematically making almost every
  decision point look much newer than it actually is. **Fix:** keep
  `--follow`, drop `--reverse`, fetch commits newest-first, and take the
  last line as the oldest/introducing commit.
- `git tag --contains <commit>` returns *all* tags containing the commit,
  including many non-release CI/dev tags in the CERTCC/SSVC repo (e.g.
  `2025.09.1223`, `fix-834-attempt-1`, `testingtag`, `old_json`). Only tags
  present in `releases_sorted.txt` (i.e. actual GitHub Releases) are
  considered.
- Some decision points appear to change `name`/`key` across versions while
  keeping the same conceptual identity (e.g. CVSS `E` key: "Exploitability"
  → "Exploit Code Maturity" → "Exploit Maturity"). The extraction keeps
  these as separate rows grouped by file/key — this is exactly the
  `namespace -> key -> version` structure the JSON output uses.
- Even with the `--follow` fix, commit dates in this repo often reflect
  when spec content was **retroactively encoded** as Python objects, not
  when it was truly first defined/published (e.g. CVSS v1/v2/v3 metrics
  encoded in a single Nov 2023 commit, even though CVSS v1 dates to 2005).
  The `retroactive` field in the JSON output is a manually curated,
  best-effort judgment call per introducing-commit — see
  `4_build_structured_data.py`'s `COMMIT_NOTES` table and `../README.md`.

# SSVC Decision Point Version History (Research Notes)

Research task for CSAF 2.1 test **6.3.13 "Usage of Non-Latest SSVC Decision
Point Version"**. Goal: understand how/when the individual SSVC decision
point *versions* became available, based on the actual commit/release
history of [CERTCC/SSVC](https://github.com/CERTCC/SSVC) (submodule checked
out at `ssvc/` in this repo, all tags fetched).

This is groundwork for a later pipeline that will generate the machine-readable
"since when available" list consumed by the JS/WASM library
(`ssvc_object_registry.json`). It is **not** that pipeline yet.

Two artifacts are produced here:

- This document — narrative summary, methodology, timeline, caveats.
- [`decision-points-by-namespace-key-version.json`](./decision-points-by-namespace-key-version.json)
  — the same data keyed **`namespace -> key -> version`**, including the
  first release tag/date and a `retroactive` flag + note for each version
  (see [Retroactive vs. genuine dates](#retroactive-vs-genuine-dates) below).

## ⚠️ Correction (superseded an earlier draft of this document)

An earlier draft of this analysis used `git log -G<pattern> --follow
--reverse` to find the oldest commit introducing each decision-point
version. **`--follow` combined with `--reverse` silently returns no output**
in the git version used here (2.50.1) whenever a file was renamed/moved —
which applies to almost every file under `src/ssvc/decision_points/`, since
the whole package was reorganized multiple times (e.g.
`decision_points/exploitation.py` → `decision_points/ssvc_/exploitation.py`
→ `decision_points/ssvc/exploitation.py`). The earlier draft silently fell
back to the *post-rename* history only, which made almost every decision
point look far newer than it actually is (e.g. it dated the entire `ssvc`
namespace to 2025-06-17, when in fact the bulk of it dates to 2023-10-16).

**Fix:** omit `--reverse`, fetch commits newest-first, and take the last
line ourselves as the introducing commit (see
`scripts/2_analyze_git_history.py`). All data below and in the JSON file
reflect the corrected method.

## Method

1. The CERTCC/SSVC repo models every decision point value set as a Python
   object (`CvssDecisionPoint`, `SsvcDecisionPoint`, …) with an explicit
   `version="x.y.z"` field, living under `src/ssvc/decision_points/`.
2. For each of the 109 versioned decision-point objects found in the current
   tree, `git log --follow -G'version="x.y.z"' -- <file>` finds every commit
   that changed the presence of that exact version literal in that file
   (across renames); the **oldest** (last in the newest-first list) is taken
   as the introducing commit.
3. That commit is cross-referenced against the **published GitHub Releases**
   for CERTCC/SSVC (via the GitHub REST API, which gives true
   `published_at` timestamps, not just tag-creation dates — several tags,
   e.g. `v1.0`/`v1.1`/`v2.0`, were only turned into GitHub Releases in 2023
   even though they represent 2019–2021 spec versions).
4. For each decision-point version, the **first release that contains** the
   introducing commit (earliest `published_at`) is reported.

Scripts used are included under [`./scripts/`](./scripts) for reuse by the
follow-up pipeline task. Run
`./scripts/run_all.sh /path/to/SSVC/checkout <output-dir>` to reproduce this
data (e.g. point it at this repo's `ssvc/` git submodule, which is a full
clone of CERTCC/SSVC with all tags fetched).

## Retroactive vs. genuine dates

Even with the `--follow` fix, **git/commit history of the code repo does
not equal "when the decision point was first defined by SSVC/CVSS/NIST"**.
The structured, versioned Python object model was itself built up in a
handful of large commits that each *encoded pre-existing spec content* that
had existed for years beforehand, alongside some commits that really are
"born here" genuine definitions/updates. Both
`decision-points-by-namespace-key-version.json` and the table below record,
per introducing commit, a `retroactive` classification:

| Introducing commit | Date | `retroactive` | Reasoning |
|---|---|---|---|
| `Add SSVC python module (#342)` | 2023-10-16 | **true** | First encoding of core SSVC decision points (Exploitation, Automatable, System Exposure, …) as Python objects. These already existed in the SSVC spec (v1.0/2019-12, v1.1/2020-09, v2.0/2021-05, or v2.1/2023-07, depending on the version) well before this commit. |
| `Add CVSS-based (v1, v2, v3) decision points as python classes (#343)` | 2023-11-07 | **true** | Encodes CVSS v1 (2005)/v2 (2007)/v3.0 (2015)/v3.1 (2019) metrics, all pre-dating this repo by years. |
| `Add CVSSv4 Decision Points (#377)` | 2023-11-17 | false | CVSS v4.0 was published by FIRST in **November 2023** — the same month as this commit. Likely close to the real-world availability date, not a backfill. |
| `Add python decision points for critical software, high value assets, and in KEV (#346)` | 2023-10-17 | false | No earlier public definition known for these CISA-related/example decision points; treated as their genuine origin. |
| `Add CVSS vector elements as decision point docs (#691)` | 2025-02-20 | **true** | VC/VI/VA are part of the CVSS v4.0 spec (Nov 2023); only added as separate Python objects here, >1 year later. |
| `Realign Safety decision points IEC 61508 (#439)` | 2024-02-07 | false | Genuine content update at the time of the commit. |
| `Add CISA custom decision points (#427)` | 2024-02-05 | false | No earlier public definition known; treated as genuine origin. |
| `add decision points to cover probability ranges and quantiles` | 2025-08-28 | false | Newly authored generic/basic decision points, not a backfill. |
| `rev human impact decision point to reflect that None and Degraded were combined a while back.` | 2025-08-12 | **true** | Commit message itself states the conceptual change happened earlier; code only caught up here. |
| `bump public_well_being version` | 2025-08-13 | false | Genuine version bump at the time of the commit. |
| `add safety v2.0.1 with updated definitions that do not include HTML` | 2025-09-15 | false | Genuine (mostly cosmetic/formatting) update at the time of the commit. |
| `Update Exploitation:PoC definition (#442)` | 2024-02-13 | false | Genuine definition update at the time of the commit. |
| `Tool to auto populate documentation examples for decision point objects (#370)` | 2023-11-14 | **null/unclear** | Pickaxe matched due to a formatting/tooling change; unclear whether the version is genuinely new here or existed in a different literal form beforehand. Low confidence. |
| `Split CVSS equivalence sets (#685)` | 2025-02-18 | **true** | CVSS v4 equivalence sets are conceptually part of the CVSS v4.0 spec (Nov 2023); this commit only splits an existing compound object into six separate ones. |

Upstream itself acknowledges this ambiguity — see
[CERTCC/SSVC#1219](https://github.com/CERTCC/SSVC/issues/1219), filed by
`tziemek` to get clarity on "when a decision point becomes
official/released".

### Recommendation for the follow-up pipeline task

Use `first_release_date` from the JSON as the practical "since available"
timestamp for machine consumption, but surface the `retroactive` flag/note
wherever the pipeline's output is human-reviewed or used to justify a
CSAF 6.3.13 rule outcome — a `retroactive: true` entry means the *actual*
availability is likely materially earlier than the recorded release date.

## Release timeline used (GitHub `published_at`, not tag-creation date)

| Tag | Published |
|---|---|
| v1.0 | 2023-04-12 (represents 2019-12 spec) |
| v1.1 | 2023-04-12 (represents 2020-09 spec) |
| v2.0 | 2023-04-12 (represents 2021-05 spec) |
| v2.1 | 2023-07-17 |
| v2.1.1 | 2023-09-01 |
| v2024.3 | 2024-03-08 |
| v2024.3.1 … v2024.3.9 | 2024-04-12 … 2025-01-16 |
| v2025.2 | 2025-02-27 |
| v2025.3, v2025.3.1–3 | 2025-03-26 … 2025-04-03 |
| v2025.6 | 2025-06-18 |
| v2025.9, v2025.9.1–5 | 2025-09-17 … 2026-01-26 |
| v2026.6, v2026.6.1 | 2026-06-11 / 2026-06-12 |
| 2026.7.0 | 2026-07-20 |

## Full per-decision-point-version data (109 entries)

Grouped by namespace (Python package under `src/ssvc/decision_points/`), then
by file, in ascending version order. See
`decision-points-by-namespace-key-version.json` for the same data keyed by
`namespace -> key -> version` with the `retroactive` flag attached to each
entry.

### `ssvc` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Virulence (V) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Automatable (A) | 2.0.0 | 2023-11-14 | v2024.3 | 2024-03-08 |
| Critical Software (CS) | 1.0.0 | 2023-10-17 | v2024.3 | 2024-03-08 |
| Exploitation (E) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Exploitation (E) | 1.1.0 | 2024-02-13 | v2024.3 | 2024-03-08 |
| High Value Asset (HVA) | 1.0.0 | 2023-10-17 | v2024.3 | 2024-03-08 |
| Mission and Well-Being Impact (MWI) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Human Impact (HI) | 2.0.0 | 2024-02-05 | v2024.3 | 2024-03-08 |
| Human Impact (HI) | 2.0.1 | 2024-02-07 | v2024.3 | 2024-03-08 |
| Human Impact (HI) | 2.0.2 | 2025-08-12 | v2025.9 | 2025-09-17 |
| Mission Impact (MI) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Mission Impact (MI) | 2.0.0 | 2023-10-17 | v2024.3 | 2024-03-08 |
| Public Well-Being Impact (PWI) | 1.1.0 | 2025-08-13 | v2025.9 | 2025-09-17 |
| Public Safety Impact (PSI) | 2.0.0 | 2024-02-05 | v2024.3 | 2024-03-08 |
| Public Safety Impact (PSI) | 2.0.1 | 2024-02-07 | v2024.3 | 2024-03-08 |
| Public Value Added (PVA) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Report Credibility (RC) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Report Public (RP) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Safety Impact (SI) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Safety Impact (SI) | 2.0.0 | 2024-02-07 | v2024.3 | 2024-03-08 |
| Safety Impact (SI) | 2.0.1 | 2025-09-15 | v2025.9 | 2025-09-17 |
| Supplier Cardinality (SC) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Supplier Contacted (SCON) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Supplier Engagement (SE) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Supplier Involvement (SINV) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| System Exposure (EXP) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| System Exposure (EXP) | 1.0.1 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Technical Impact (TI) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Utility (U) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Utility (U) | 1.0.1 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Value Density (VD) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |

### `cvss` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Access Complexity (AC) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Access Complexity (AC) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Attack Complexity (AC) | 3.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Attack Complexity (AC) | 3.0.1 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Attack Requirements (AT) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Access Vector (AV) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Access Vector (AV) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Attack Vector (AV) | 3.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Attack Vector (AV) | 3.0.1 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Authentication (Au) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Authentication (Au) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Availability Impact (A) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Availability Impact (A) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Availability Impact to the Vulnerable System (VA) | 3.0.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Availability Requirement (AR) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Availability Requirement (AR) | 1.1.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Availability Requirement (AR) | 1.1.1 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Collateral Damage Potential (CDP) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Collateral Damage Potential (CDP) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Confidentiality Impact (C) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Confidentiality Impact (C) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Confidentiality Impact to the Vulnerable System (VC) | 3.0.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Confidentiality Requirement (CR) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Confidentiality Requirement (CR) | 1.1.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Confidentiality Requirement (CR) | 1.1.1 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Equivalence Set 1 (EQ1) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Equivalence Set 2 (EQ2) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Equivalence Set 3 (EQ3) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Equivalence Set 4 (EQ4) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Equivalence Set 5 (EQ5) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Equivalence Set 6 (EQ6) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Exploitability (E) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Exploitability (E) | 1.1.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Exploit Code Maturity (E) | 1.2.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Exploit Maturity (E) | 2.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Impact Bias (IB) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Impact (I) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Impact (I) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Impact to the Vulnerable System (VI) | 3.0.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Integrity Requirement (IR) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Requirement (IR) | 1.1.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Integrity Requirement (IR) | 1.1.1 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Privileges Required (PR) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Privileges Required (PR) | 1.0.1 | 2023-11-17 | v2024.3 | 2024-03-08 |
| CVSS Qualitative Severity Rating Scale (QS) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Remediation Level (RL) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Remediation Level (RL) | 1.1.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Report Confidence (RC) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Report Confidence (RC) | 1.1.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Report Confidence (RC) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Scope (S) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Availability Impact to the Subsequent System (SA) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Confidentiality Impact to the Subsequent System (SC) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Integrity Impact to the Subsequent System (SI) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Automatable (AU) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Provider Urgency (U) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Recovery (R) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Safety (SF) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Value Density (V) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Vulnerability Response Effort (RE) | 1.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Target Distribution (TD) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Target Distribution (TD) | 1.1.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| User Interaction (UI) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| User Interaction (UI) | 2.0.0 | 2023-11-17 | v2024.3 | 2024-03-08 |

### `cisa` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| In KEV (KEV) | 1.0.0 | 2023-10-17 | v2024.3 | 2024-03-08 |
| Mission Prevalence (MP) | 1.0.0 | 2024-02-05 | v2024.3 | 2024-03-08 |
| Publicly Exposed (PE) | 1.0.0 | 2023-10-17 | v2024.3 | 2024-03-08 |

### `nist` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Probability Scale in 5 weighted levels, ascending (P_5X) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |

### `basic` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Boundary Proximity (BP) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| CIS-CTI Words of Estimative Probability (CIS_WEP) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Probability Scale in 5 equal levels, ascending (P_5A) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Probability Scale in 5 weighted levels, ascending (P_5W) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Probability Scale in 2 equal levels, ascending (P_2A) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Median Split (MEDIAN) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Quartiles (QUARTILES) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |
| Quintiles (QUINTILES) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |

### `example` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Humidity Value above 40% (H) | 1.0.0 | 2023-10-17 | v2024.3 | 2024-03-08 |
| Weather Forecast (W) | 1.0.0 | 2023-10-16 | v2024.3 | 2024-03-08 |

## Open questions for the pipeline task

1. Should pre-2023 decision points use the retro-dated GitHub Release
   publish dates (`v1.0`/`v1.1`/`v2.0` = 2023-04-12) or their *intended*
   spec dates (2019-12 / 2020-09 / 2021-05)? The GitHub API only exposes the
   former; the latter would require a manually curated override table.
2. Do we key on GitHub Release `published_at` (a point-in-time event) or on
   the underlying commit date (when the change actually landed on `main`)?
   For test 6.3.13 purposes, the release date is probably more defensible
   since that's when the version became publicly *available*.
3. Several files define multiple versions of a decision point that changed
   both `key` and `name` over time (e.g. CVSS "Exploitability" → "Exploit
   Code Maturity" → "Exploit Maturity", all key `E`). The pipeline needs a
   stable identity per decision point (likely: namespace + key, or
   namespace + name-lineage) independent of the display name — this is
   exactly what `decision-points-by-namespace-key-version.json` already
   does (`namespace -> key -> version`).
4. The `retroactive` classification in the JSON is a best-effort judgment
   call based on public knowledge of when CVSS/SSVC/NIST specs were
   actually published, not something derivable purely from this repo. It
   should be reviewed/refined by someone closer to the SSVC spec history
   before being relied upon.

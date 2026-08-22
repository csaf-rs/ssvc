# SSVC Decision Point Version History (Research Notes)

Research task for CSAF 2.1 test **6.3.13 "Usage of Non-Latest SSVC Decision Point
Version"**. Goal: understand how/when the individual SSVC decision point
*versions* became available, based on the actual commit/release history of
[CERTCC/SSVC](https://github.com/CERTCC/SSVC) (submodule checked out at
`ssvc/` in this repo, all tags fetched).

This is groundwork for a later pipeline that will generate the machine-readable
"since when available" list consumed by the JS/WASM library
(`ssvc_object_registry.json`). It is **not** that pipeline yet.

## Method

1. The CERTCC/SSVC repo models every decision point value set as a Python
   object (`CvssDecisionPoint`, `SsvcDecisionPoint`, …) with an explicit
   `version="x.y.z"` field, living under `src/ssvc/decision_points/`.
2. For each of the 109 versioned decision-point objects found in the current
   tree, I used `git log -G'version="x.y.z"' -- <file>` to find the **oldest
   commit** that introduced that exact version literal in that file.
3. I cross-referenced that commit against the **published GitHub Releases**
   for CERTCC/SSVC (via the GitHub REST API, which gives true
   `published_at` timestamps, not just tag-creation dates — several tags,
   e.g. `v1.0`/`v1.1`/`v2.0`, were only turned into GitHub Releases in 2023
   even though they represent 2019–2021 spec versions).
4. For each decision-point version I report the **first release that
   contains** the introducing commit (earliest `published_at`).

Scripts used are ad-hoc (AST-based extraction + git plumbing) and are
included under `./scripts/` for reuse by the follow-up pipeline task. Run
`./scripts/run_all.sh /path/to/SSVC/checkout` to reproduce this data (e.g.
point it at this repo's `ssvc/` git submodule, which is a full clone of
CERTCC/SSVC with all tags fetched).

## ⚠️ Key finding / caveat

**Git/commit history of the code repo does not equal "when the decision
point was first defined by SSVC".** The structured, versioned Python object
model is a relatively recent addition to the CERTCC/SSVC repo:

| Milestone | Date | Release |
|---|---|---|
| SSVC v1.0 spec (original paper) | 2019-12 (release published retroactively 2023-04-12) | `v1.0` |
| SSVC v1.1 | 2020-09 (release published 2023-04-12) | `v1.1` |
| SSVC v2.0 | 2021-05 (release published 2023-04-12) | `v2.0` |
| SSVC v2.1 / v2.1.1 | 2023-07 / 2023-09 | `v2.1`, `v2.1.1` |
| **First bulk import of CVSS metrics as versioned Python objects** (`attack_vector.py`, `attack_complexity.py`, … with explicit `version=` strings for v1.0.0–v3.0.1 style CVSS history) | 2023-11-07 | shipped in **v2024.3** (2024-03-08) |
| CVSS v4 equivalence sets (EQ1–EQ6), VC/VI/VA, qualitative severity | 2025-02-18 → 2025-02-21 | shipped in **v2025.2** (2025-02-27) |
| **First appearance of the core `ssvc` namespace** (Exploitation, Automatable, Technical Impact, Mission Impact, System Exposure, Human Impact, Safety Impact, Utility, etc. — i.e. the decision points actually referenced by the CSAF SSVC provider URN scheme) as importable Python objects with `version=` | 2025-06-17 | shipped in **v2025.9** (2025-09-17) |
| `basic`, `nist` helper namespaces (probability scales, quantiles) | 2025-08-28 / 2025-09-09 / 2025-09-12 | shipped in **v2025.9** (2025-09-17) |

Practically, this means: for the *older* decision-point values (e.g. CVSS
Attack Vector 1.0.0/2.0.0, or SSVC Exploitation 1.0.0), the "first seen in
code" date in the table below reflects when CERTCC **retro-encoded**
historical spec content into this repository — **not** when that value was
first published/used by the community. The upstream project itself
acknowledges this ambiguity (see
[CERTCC/SSVC#1219](https://github.com/CERTCC/SSVC/issues/1219), which
`tziemek` filed to get clarity on "when a decision point becomes
official/released").

For decision points that were only added to the object model in 2025 (the
entire `ssvc` namespace!), the commit/release date is a reasonable proxy for
"since when machine-readable", but a poor proxy for "since when the SSVC
methodology defined this value" (most of these existed conceptually since
SSVC v1.0/v2.0 in markdown-only form).

### Recommendation for the follow-up pipeline task

Any "since available" timestamp derived purely from `git log` on
`src/ssvc/decision_points/` will systematically **understate** the true age
of pre-2023 decision points and will need a documented fallback/override
list for those, sourced from the actual dated SSVC spec releases
(`v1.0`/2019-12, `v1.1`/2020-09, `v2.0`/2021-05, `v2.1`/2023-07,
`v2.1.1`/2023-09) rather than from the object-model commit. Post-2023
decision points (all of CVSS v3.x extras added 2023-11, CVSS v4 additions in
2025-02, and the full `ssvc` namespace added 2025-06/2025-09) are reasonably
well dated by this method since they were added to the object model close to
when they were actually introduced/released.

## Release timeline used (GitHub "published_at", not tag-creation date)

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
by file, in ascending version order.

### `ssvc` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Virulence (V) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Automatable (A) | 2.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Critical Software (CS) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Exploitation (E) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Exploitation (E) | 1.1.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| High Value Asset (HVA) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Mission and Well-Being Impact (MWI) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Human Impact (HI) | 2.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Human Impact (HI) | 2.0.1 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Human Impact (HI) | 2.0.2 | 2025-08-12 | v2025.9 | 2025-09-17 |
| Mission Impact (MI) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Mission Impact (MI) | 2.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Public Well-Being Impact (PWI) | 1.1.0 | 2025-08-13 | v2025.9 | 2025-09-17 |
| Public Safety Impact (PSI) | 2.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Public Safety Impact (PSI) | 2.0.1 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Public Value Added (PVA) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Report Credibility (RC) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Report Public (RP) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Safety Impact (SI) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Safety Impact (SI) | 2.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Safety Impact (SI) | 2.0.1 | 2025-09-15 | v2025.9 | 2025-09-17 |
| Supplier Cardinality (SC) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Supplier Contacted (SCON) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Supplier Engagement (SE) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Supplier Involvement (SINV) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| System Exposure (EXP) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| System Exposure (EXP) | 1.0.1 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Technical Impact (TI) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Utility (U) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Utility (U) | 1.0.1 | 2025-06-17 | v2025.9 | 2025-09-17 |
| Value Density (VD) | 1.0.0 | 2025-06-17 | v2025.9 | 2025-09-17 |

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
| Equivalence Set 1 (EQ1) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Equivalence Set 2 (EQ2) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Equivalence Set 3 (EQ3) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Equivalence Set 4 (EQ4) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Equivalence Set 5 (EQ5) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Equivalence Set 6 (EQ6) | 1.0.0 | 2025-02-18 | v2025.2 | 2025-02-27 |
| Exploitability (E) | 1.0.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Exploitability (E) | 1.1.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Exploit Code Maturity (E) | 1.2.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Exploit Maturity (E) | 2.0.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Impact Bias (IB) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Impact (I) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Impact (I) | 2.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Impact to the Vulnerable System (VI) | 3.0.0 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Integrity Requirement (IR) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Integrity Requirement (IR) | 1.1.0 | 2023-11-17 | v2024.3 | 2024-03-08 |
| Integrity Requirement (IR) | 1.1.1 | 2025-02-20 | v2025.2 | 2025-02-27 |
| Privileges Required (PR) | 1.0.0 | 2023-11-07 | v2024.3 | 2024-03-08 |
| Privileges Required (PR) | 1.0.1 | 2023-11-17 | v2024.3 | 2024-03-08 |
| CVSS Qualitative Severity Rating Scale (QS) | 1.0.0 | 2025-02-21 | v2025.2 | 2025-02-27 |
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
| In KEV (KEV) | 1.0.0 | 2025-08-05 | v2025.9 | 2025-09-17 |
| Mission Prevalence (MP) | 1.0.0 | 2025-06-18 | v2025.9 | 2025-09-17 |
| Publicly Exposed (PE) | 1.0.0 | 2026-06-11 | v2026.6 | 2026-06-11 |

### `nist` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Probability Scale in 5 weighted levels, ascending (P_5X) | 1.0.0 | 2025-09-09 | v2025.9 | 2025-09-17 |

### `basic` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Boundary Proximity (BP) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| CIS-CTI Words of Estimative Probability (CIS_WEP) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Probability Scale in 5 equal levels, ascending (P_5A) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Probability Scale in 5 weighted levels, ascending (P_5W) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Probability Scale in 2 equal levels, ascending (P_2A) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Median Split (MEDIAN) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Quartiles (QUARTILES) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |
| Quintiles (QUINTILES) | 1.0.0 | 2025-08-28 | v2025.9 | 2025-09-17 |

### `example` namespace

| Decision Point (key) | Version | First seen in code (commit date) | First shipped release | Release date |
|---|---|---|---|---|
| Humidity Value above 40% (H) | 1.0.0 | 2025-09-12 | v2025.9 | 2025-09-17 |
| Weather Forecast (W) | 1.0.0 | 2025-09-12 | v2025.9 | 2025-09-17 |

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
   both `key` and `name` over time (e.g. CVSS 'Exploitability' → 'Exploit
   Code Maturity' → 'Exploit Maturity', all key `E`). The pipeline needs a
   stable identity per decision point (likely: namespace + key, or
   namespace + name-lineage) independent of the display name.
4. `SSVC` core namespace decision points (Exploitation, Automatable, etc.)
   all show the *same* introduction date (2025-06-17) because they were
   added to the object model in one bulk commit — this is a strong signal
   these dates are an artifact of the repo refactor, not real availability
   dates, and will need special-casing.


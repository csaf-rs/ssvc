import json, sys

INPUT_FILE = sys.argv[1] if len(sys.argv) > 1 else "dps_analyzed_full.txt"

# Classification of the *introducing commit* for each decision point version:
# is encoding it in this Python object model a retroactive backfill of
# already-existing SSVC/CVSS/NIST spec content, or a genuine "born here"
# definition/update? Keyed by commit subject (stable across files since we
# observed each subject maps to one clear case). See README.md for sources.
COMMIT_NOTES = {
    "Add SSVC python module (#342)": (
        True,
        "First encoding of core SSVC decision points as Python objects. The "
        "underlying decision point already existed in the SSVC spec/docs "
        "(v1.0/2019-12, v1.1/2020-09, v2.0/2021-05 or v2.1/2023-07 depending "
        "on the version number) well before this commit."),
    "Add CVSS-based (v1, v2, v3) decision points as python classes (#343)": (
        True,
        "Encodes CVSS v1 (2005)/v2 (2007)/v3.0 (2015)/v3.1 (2019) metric "
        "definitions, all pre-dating this repository, as Python objects."),
    "Add CVSSv4 Decision Points (#377)": (
        False,
        "CVSS v4.0 was published by FIRST in November 2023 — the same month "
        "as this commit. Likely close to the real-world availability date "
        "rather than a retroactive backfill."),
    "Add python decision points for critical software, high value assets, and in KEV (#346)": (
        False,
        "First introduction of these CISA-related and example decision "
        "points; no earlier public definition is known, so this is treated "
        "as their genuine origin."),
    "Add CVSS vector elements as decision point docs (#691)": (
        True,
        "VC/VI/VA are part of the CVSS v4.0 spec (published Nov 2023); this "
        "commit only adds them as separate Python objects more than a year "
        "later."),
    "Realign Safety decision points IEC 61508 (#439)": (
        False,
        "Genuine content update realigning the Safety Impact decision point "
        "with the IEC 61508 standard at the time of the commit."),
    "Add CISA custom decision points (#427)": (
        False,
        "First introduction of these CISA-specific decision points; no "
        "earlier public definition is known, so this is treated as their "
        "genuine origin."),
    "add decision points to cover probability ranges and quantiles": (
        False,
        "Newly authored generic/basic decision points, not a backfill of "
        "pre-existing SSVC spec content."),
    "rev human impact decision point to reflect that None and Degraded were combined a while back.": (
        True,
        "Commit message states the conceptual merge of values happened "
        "earlier; this commit only updates the code to match, so the "
        "version's *real* availability likely pre-dates this commit."),
    "bump public_well_being version": (
        False,
        "Genuine version bump at the time of the commit."),
    "add safety v2.0.1 with updated definitions that do not include HTML": (
        False,
        "Genuine (mostly cosmetic/formatting) content update at the time of "
        "the commit."),
    "Update Exploitation:PoC definition (#442)": (
        False,
        "Genuine definition update at the time of the commit."),
    "Tool to auto populate documentation examples for decision point objects (#370)": (
        None,
        "Pickaxe matched this commit because of a formatting/tooling change "
        "touching the file; it is unclear whether the version itself is "
        "genuinely new here or already existed in a different literal form "
        "beforehand. Treat this date with low confidence."),
    "Split CVSS equivalence sets (#685)": (
        True,
        "CVSS v4 equivalence sets are conceptually part of the CVSS v4.0 "
        "spec (Nov 2023); this commit only splits an existing compound "
        "object into separate per-equivalence-set Python objects."),
}

tree = {}
with open(INPUT_FILE) as f:
    for line in f:
        line = line.rstrip("\n")
        if not line:
            continue
        namespace, path, varname, name, key, version, commit, commit_date, subject, tag, tag_date = line.split("|")
        retro, note = COMMIT_NOTES.get(subject, (None, "Unclassified commit; no retroactivity note available."))
        ns_node = tree.setdefault(namespace, {})
        key_node = ns_node.setdefault(key, {"versions": {}})
        key_node["versions"][version] = {
            "name": name.strip(),
            "source_file": path,
            "first_commit": commit,
            "first_commit_date": commit_date,
            "first_commit_subject": subject,
            "first_release_tag": tag if tag != "None" else None,
            "first_release_date": tag_date if tag_date != "None" else None,
            "retroactive": retro,
            "retroactive_note": note,
        }

print(json.dumps(tree, indent=2, sort_keys=True))

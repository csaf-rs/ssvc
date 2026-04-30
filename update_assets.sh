#!/usr/bin/env bash

# This script is intended to update LOCAL assets (i.e. from git submodules). It SHOULD be run after
# each checkout or similar operation to make sure that assets are in sync with the submodules.
# Failing to run this script before the build might result in unexpected behavior due to mismatches
# between embedded assets and the library version!

set -euo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

rsync -c ssvc/data/schema/v2/DecisionPoint_2_0_0.schema.json assets/DecisionPoint_2_0_0.schema.json
rsync -c ssvc/data/schema/v2/SelectionList_2_0_0.schema.json assets/SelectionList_2_0_0.schema.json
rsync -cr --delete ssvc/data/json/decision_points/ assets/ssvc_decision_points/

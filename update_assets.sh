#!/usr/bin/env bash

set -euo pipefail

# This script is intended to update LOCAL assets (i.e. from git submodules). It SHOULD be run after
# each checkout or similar operation to make sure that assets are in sync with the submodules.
# Failing to run this script before the build might result in unexpected behavior due to mismatches
# between embedded assets and the library version!

# Get the directory of this script (which is part of the main repo, not some submodule)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

# Check if the submodule was checked out before attempting to copy assets
SOURCE_ROOT="ssvc"
if [[ ! -d "$SOURCE_ROOT/data" ]]; then
  echo "SSVC submodule is not initialized. Run: git submodule update --init --recursive" >&2
  exit 1
fi

# Sync relevant files
echo "Updating assets..."

if rsync -c "$SOURCE_ROOT/data/schema/v2/DecisionPoint_2_0_0.schema.json" assets/DecisionPoint_2_0_0.schema.json; then
  echo "DecisionPoint schema updated"
else
  echo "ERROR: Failed to update DecisionPoint schema" >&2
  exit 1
fi

if rsync -c "$SOURCE_ROOT/data/schema/v2/SelectionList_2_0_0.schema.json" assets/SelectionList_2_0_0.schema.json; then
  echo "SelectionList schema updated"
else
  echo "ERROR: Failed to update SelectionList schema" >&2
  exit 1
fi

if rsync -cr --delete "$SOURCE_ROOT/data/json/decision_points/" assets/ssvc_decision_points/; then
  echo "Decision points synchronized"
else
  echo "ERROR: Failed to synchronize decision points" >&2
  exit 1
fi

echo "Assets updated successfully"

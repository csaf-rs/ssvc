#!/usr/bin/env bash

set -euo pipefail

# This script is intended to update LOCAL assets (i.e. from git submodules). It SHOULD be run after
# each checkout or similar operation to make sure that assets are in sync with the submodules.
# Failing to run this script before the build might result in unexpected behavior due to mismatches
# between embedded assets and the library version!

SOURCE_ROOT="SSVC"

if [[ ! -d "$SOURCE_ROOT/data" ]]; then
	echo "SSVC submodule is not initialized. Run: git submodule update --init --recursive" >&2
	exit 1
fi

rsync -c "$SOURCE_ROOT/data/schema/v2/DecisionPoint_2_0_0.schema.json" assets/DecisionPoint_2_0_0.schema.json
rsync -c "$SOURCE_ROOT/data/schema/v2/SelectionList_2_0_0.schema.json" assets/SelectionList_2_0_0.schema.json
rsync -cr --delete "$SOURCE_ROOT/data/json/decision_points/" assets/ssvc_decision_points/

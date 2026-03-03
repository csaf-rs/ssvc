# ssvcAsset Sources & Licenses

This library is meant to be published as rust crate(s). For the sake of successful publishing and
reproducibility, we have to bundle/vendor relevant (external) assets. These assets are described
within this `README.md`.

## git-based Assets

### SSVC Resources

- `ssvc_decision_points/**`
- `DecisionPoint_2_0_0.schema.json`

See https://github.com/CERTCC/SSVC/tree/main/data/json/decision_points for information regarding these files and
https://github.com/CERTCC/SSVC/blob/main/data/LICENSE for information about licenses.
They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

- `SelectionList_2_0_0.schema.json`

See https://github.com/CERTCC/SSVC/blob/main/data/schema/v2/SelectionList_2_0_0.schema.json for this schema file and
https://github.com/CERTCC/SSVC/blob/main/data/LICENSE for information about licenses.
This is a 1:1 copy from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

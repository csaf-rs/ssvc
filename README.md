# SSVC Rust Implementation

A Rust implementation of the **SSVC (Stakeholder-Specific Vulnerability Categorization)** specification.
SSVC is a framework for prioritizing software vulnerability remediation efforts. It helps stakeholders make informed decisions about which vulnerabilities to address first by considering factors like vulnerability severity, the stakeholder's position in the ecosystem, and their specific constraints.
Learn more at the [official SSVC documentation](https://certcc.github.io/SSVC/).

# Features

This library provides validation and processing of SSVC decision points and selection lists, with support for SSVC namespaces and extensions.
It features full serde support. The library supports both native Rust usage and WebAssembly (WASM) bindings for JavaScript/web applications.

## Installation

Add to your project:

```bash
cargo add ssvc
```

## MSRV

1.85.0

## Examples

### Rust

```rust
use ssvc::selection_list::SelectionList;
use ssvc::validate_selection_list;

// Parse a selection list
let selection_list: SelectionList = serde_json::from_str(json_data)?;

// Validate the selection list
let result = validate_selection_list(&selection_list, false);

if result.success {
    println!("Selection list is valid!");
} else {
    for error in result.errors {
        println!("Validation error: {}", error.message);
    }
}
```

### WebAssembly

#### Build

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack
# Build for web
wasm-pack build --target web --out-dir pkg -- --features wasm
```

#### Usage

```javascript
import * as wasm from './pkg/ssvc.js';

const jsonData = {...}; // Your SSVC selection list

try {
  const result = wasm.validateSelectionList(JSON.stringify(jsonData), false);
  if (result.success) {
    console.log("Valid SSVC data");
  } else {
    console.log("Validation errors:", result.errors);
  }
} catch (error) {
  console.error("Error:", error);
}
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) file for details.

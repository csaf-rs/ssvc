# @csaf-rs/ssvc

A WebAssembly build of the **SSVC (Stakeholder-Specific Vulnerability Categorization)** Rust implementation, for use with JavaScript/TypeScript.
SSVC is a framework for prioritizing software vulnerability remediation efforts. It helps stakeholders make informed decisions about which vulnerabilities to address first by considering factors like vulnerability severity, the stakeholder's position in the ecosystem, and their specific constraints.
Learn more at the [official SSVC documentation](https://certcc.github.io/SSVC/).

For the Rust crate, source code, and full project documentation, see the [csaf-rs/ssvc on GitHub](https://github.com/csaf-rs/ssvc).

## Features

This package provides validation and processing of SSVC decision points and selection lists, with support for SSVC namespaces and extensions.

## Installation

```bash
npm install @csaf-rs/ssvc
```

## Usage

The WASM module must be initialized once before its functions can be used:

```javascript
import init, { validateSelectionList } from '@csaf-rs/ssvc';

await init();

const jsonData = {...}; // Your SSVC selection list

try {
  const result = validateSelectionList(JSON.stringify(jsonData), false);
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

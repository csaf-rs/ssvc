# ssvc
Implementation of the SSVC specification in Rust

## WASM Build

Build the WebAssembly artifact with:

```
cargo install wasm-pack
wasm-pack build --target web --out-dir pkg -- --features wasm
```

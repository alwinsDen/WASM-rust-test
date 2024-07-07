# WASM Rust Dev Test

## Pre-requisites

#### Rust Installer
Install Rust using the following command:
```commandline
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Install WASM pack
Install the wasm-pack using Cargo:
```commandline
cargo install wasm-pack
```

## Building the WASM binary web target
For NPM users [post-building]:
```commandline
yarn run serve
```

Use wasm-pack to build the WASM binary targeting the web:
```commandline
wasm-pack build --target web
```

## Resources
[rustwasm.github.io](https://rustwasm.github.io/wasm-bindgen/examples/index.html)

[MDN web docs](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)
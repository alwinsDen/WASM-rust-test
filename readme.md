# WASM Rust Dev Test

### Pre-requisites

#### Rust Installer
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Install WASM pack
```shell
cargo install wasm-pack
```

## Building the WASM binary web target
```shell
wasm-pack build --target web
```
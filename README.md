# Build and Run (Rust → WASM, no bundler)

Minimal dependencies; plain HTML/JS.

## Requirements
- Rust (`rustup`, `cargo`): https://rustup.rs
- wasm-bindgen CLI: `cargo install wasm-bindgen-cli`
- Any static server (e.g., Python)

## One-time setup (PowerShell)
```powershell
# Open a NEW PowerShell after installing rustup
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

## Build
```powershell
# From the project root
cd .\wasm-lib

# Compile Rust library to WebAssembly (release)
cargo build --release --target wasm32-unknown-unknown

# Generate browser glue into the web/ folder
wasm-bindgen --target web --out-dir ..\web .\target\wasm32-unknown-unknown\release\wasm_lib.wasm
```

Outputs in `web/`:
- `wasm_lib.js`, `wasm_lib_bg.js`, `wasm_lib_bg.wasm`

## Run locally
```powershell
cd ..\web
py -m http.server 8080  # or: python -m http.server 8080
# Then open http://localhost:8080
```

## Rebuild loop
```powershell
cd ..\wasm-lib
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir ..\web .\target\wasm32-unknown-unknown\release\wasm_lib.wasm
# Refresh the browser tab
```

## Troubleshooting
- Reopen a NEW shell after installing Rust.
- Always serve over HTTP; file:// won’t load .wasm.
- Size: LTO + opt-level=z enabled; optional: `wasm-opt -Oz`.

# Default `just` command. Runs `fmt` and checks the codebase against all regular targets/features
ci:
    cargo fmt
    cargo clippy --all-targets --all-features

# Ensure this compiles to WASM
cwasm:
    cargo clippy --target wasm32-unknown-unknown  --features wasm,rig-wasm

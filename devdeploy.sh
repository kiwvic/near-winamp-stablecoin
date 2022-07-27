rm -r neardev

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/winamp_stablecoin.wasm

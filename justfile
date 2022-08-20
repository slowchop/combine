web:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/combine.wasm
    rsync -vr assets web

host:
    python3 -m http.server

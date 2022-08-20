watch_server:
    cargo watch -s "cargo run --package server --features use-udp"

server:
    cargo run --package server --features use-udp

web:
    cd game && cargo build --release --target wasm32-unknown-unknown
    cd game && wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/combine.wasm
    rsync -vr game/assets web

host:
    cd web && python3 -m http.server

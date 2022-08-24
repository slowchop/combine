play:
    cd client && cargo run --features bevy/dynamic

build_client_release:
    cd client && cargo build --release

play_multi:
    cd client && cargo run --features bevy/dynamic -- -w 0 -s &
    cd client && cargo run --features bevy/dynamic -- -w 1 -s

watch_server:
    cargo watch -s "cargo run --package server --features use-udp"

server:
    cargo run --package server --features use-udp

web:
    cd client && cargo build --release --target wasm32-unknown-unknown
    cd client && wasm-bindgen --out-dir ../web --target web ../target/wasm32-unknown-unknown/release/towercombo.wasm
    rsync -vr client/assets web

clean_web:
    rm -fr web/*wasm* web/*.js web/*.ts assets

host:
    cd web && sfz -r --cors --coi -b 0.0.0.0 -p 8000

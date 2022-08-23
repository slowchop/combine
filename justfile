play:
    cd game && cargo run --features bevy/dynamic

play_multi:
    cd game && cargo run --features bevy/dynamic -- -w 0 -s &
    cd game && cargo run --features bevy/dynamic -- -w 1 -s

watch_server:
    cargo watch -s "cargo run --package server --features use-udp"

server:
    cargo run --package server --features use-udp

web:
    cd game && cargo build --release --target wasm32-unknown-unknown
    cd game && wasm-bindgen --out-dir ../web --target web ../target/wasm32-unknown-unknown/release/towercombo.wasm
    rsync -vr game/assets web

clean_web:
    rm -fr web/*wasm* web/*.js web/*.ts assets

host:
    cd web && sfz -r --cors --coi -b 0.0.0.0 -p 8000

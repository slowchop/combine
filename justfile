play:
    cd client && cargo run --features bevy/dynamic

build_client_release:
    cd client && cargo build --release

play_multi_dev:
    cd client && cargo run --features bevy/dynamic -- -w 0 -s -d &
    cd client && cargo run --features bevy/dynamic -- -w 1 -s -d

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

deploy_bootstrap:
    sudo apt update
    sudo apt install -y build-essential clang libssl-dev
    sudo snap install --edge --classic just
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

deploy:
    rsync -vr --exclude target * towercombo:~/towercombo
    rsync -vr --exclude target ../naia/ towercombo:~/naia
    ssh towercombo "cd towercombo && just local_deploy"

local_deploy:
    /home/gak/.cargo/bin/cargo build --release --package server --features use-webrtc

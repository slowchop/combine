play:
    cd client && cargo run --features bevy/dynamic

play_multi_dev:
    cd client && cargo run --features bevy/dynamic -- -w 0 -s -d &
    cd client && cargo run --features bevy/dynamic -- -w 1 -s -d

build_client_release:
    cd client && cargo build --release

web_clean:
    rm -fr web/*wasm* web/*.js web/*.ts assets

web_build: web_clean
    cd client && cargo build --release --target wasm32-unknown-unknown
    cd client && wasm-bindgen --out-dir ../web --target web ../target/wasm32-unknown-unknown/release/towercombo.wasm
    rsync -vr client/assets web

deploy_web: web_build
    netlify deploy --open

host:
    cd web && sfz -r --cors --coi -b 0.0.0.0 -p 8000

watch_server:
    cargo watch -s "cargo run --package server --features use-udp"

server:
    cargo run --package server --features use-udp

server_deploy_bootstrap:
    sudo apt update
    sudo apt install -y build-essential clang libssl-dev pkg-config
    sudo snap install --edge --classic just
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

server_deploy:
    rsync -vr --exclude target * towercombo:~/towercombo
    rsync -vr --exclude target ../naia/ towercombo:~/naia
    # Can't sudo with password over ssh it seems.
    # Run this on the server:
    # just server_deploy_local

server_deploy_local:
    /home/gak/.cargo/bin/cargo build --release --package server --features use-webrtc
    sudo cp deploy/towercombo.service /etc/systemd/system/
    sudo systemctl enable towercombo.service
    sudo systemctl restart towercombo.service
    journalctl -f

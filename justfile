play:
    cd client && cargo run --features bevy/dynamic --features shared/$ENV

play_multi_dev:
    #!/usr/bin/env bash
    cd client
    cargo run --features bevy/dynamic --features shared/$ENV -- -w 0 -s -d &
    cargo run --features bevy/dynamic --features shared/$ENV -- -w 1 -s -d

build_client_release:
    cd client && cargo build --release

build_client_windows:
    cd client && cargo build --release --target x86_64-pc-windows-gnu --features shared/prod
    rm -rf TowerCombo
    mkdir TowerCombo
    cp target/x86_64-pc-windows-gnu/release/towercombo.exe TowerCombo/TowerCombo.exe
    cp -r client/assets TowerCombo
    zip -r TowerCombo-Windows.zip TowerCombo


web_clean:
    rm -fr web/*wasm* web/*.js web/*.ts assets

web_build: web_clean
    cd client && cargo build --release --target wasm32-unknown-unknown --features shared/$ENV
    cd client && wasm-bindgen --out-dir ../web --target web ../target/wasm32-unknown-unknown/release/towercombo.wasm
    rsync -vr client/assets web

web_deploy: web_build
    netlify deploy --open

host:
    cd web && sfz -r --cors --coi -b 0.0.0.0 -p 8000

watch_server:
    cargo watch -s "cargo run --package server --features use-udp"

server:
    cargo run --package server --features use-udp

server_deploy_bootstrap:
    # sudo snap install --edge --classic just
    sudo apt update
    sudo apt install -y build-essential clang libssl-dev pkg-config
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

server_deploy:
    rsync -vr --exclude target * $HOST:~/towercombo
    rsync -vr --exclude target ../naia/ $HOST:~/naia
    # Can't sudo with password over ssh it seems.
    # Run this on the server:
    # just server_deploy_local

server_deploy_local:
    $HOME/.cargo/bin/cargo build --release --package server --features use-udp --features shared/prod
    sudo cp deploy/towercombo.service /etc/systemd/system/
    sudo systemctl enable towercombo.service
    sudo systemctl restart towercombo.service
    journalctl -f

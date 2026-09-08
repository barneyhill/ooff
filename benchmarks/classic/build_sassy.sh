#!/usr/bin/env bash
set -euo pipefail
export PATH="/home/ubuntu/.cargo/bin:$PATH"
root=/home/ubuntu/comparators/sassy-classic-screen-0.2.6
mkdir -p "$root/src"
cp benchmarks/classic/sassy_screen.rs "$root/src/main.rs"
cat > "$root/Cargo.toml" <<'TOML'
[package]
name = "sassy-classic-screen"
version = "0.1.0"
edition = "2024"
[dependencies]
sassy = { version = "=0.2.6", default-features = false }
oofft = { path = "/home/ubuntu/ooff" }
clap = { version = "4.5", features = ["derive"] }
serde_json = "1"
TOML
cp benchmarks/classic/sassy-Cargo.lock "$root/Cargo.lock"
cd "$root"
RUSTFLAGS='-C target-cpu=native' cargo build --locked --release -j "$(nproc)"
sha256sum target/release/sassy-classic-screen

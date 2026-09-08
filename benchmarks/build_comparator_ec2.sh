#!/usr/bin/env bash
set -euo pipefail
export PATH="/home/ubuntu/.cargo/bin:$PATH"
root="${1:-/home/ubuntu/comparators/sassy-0.2.6}"
engine_source="${2:-/home/ubuntu/ooff/benchmarks/sassy_engine.rs}"
simd="${3:-default}"
case "$simd" in default|avx512) ;; *) echo "Unsupported SIMD configuration: $simd" >&2; exit 2 ;; esac
mkdir -p "$root/src"
cp "$engine_source" "$root/src/main.rs"
cp /home/ubuntu/ooff/benchmarks/engine_driver.rs "$root/src/engine_driver.rs"
cat > "$root/Cargo.toml" <<'EOF'
[package]
name = "sassy-comparator"
version = "0.1.0"
edition = "2024"
[dependencies]
sassy = { version = "=0.2.6", default-features = false }
oofft = { path = "/home/ubuntu/ooff" }
clap = { version = "4.5", features = ["derive"] }
serde_json = "1"
EOF
if [[ "$simd" == avx512 ]]; then
    sed -i 's/default-features = false/default-features = false, features = ["avx512"]/' "$root/Cargo.toml"
fi
cd "$root"
RUSTFLAGS='-C target-cpu=native' cargo build --release -j 4
sha256sum target/release/sassy-comparator

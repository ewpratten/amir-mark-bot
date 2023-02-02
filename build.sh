#! /bin/bash
set -e

cargo build --release --target x86_64-unknown-linux-musl
docker build -t ewpratten/amir_mark_bot:latest .

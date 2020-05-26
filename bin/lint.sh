#!/usr/bin/env sh

touch src/main.rs
cargo clippy --all-targets --all-features -- -D warnings

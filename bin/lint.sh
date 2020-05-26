#!/usr/bin/env sh

touch src/lib.rs
touch src/main.rs
cargo clippy --all-targets --all-features -- -D warnings

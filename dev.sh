#!/bin/bash
cargo watch -s "cargo clippy && RUST_BACKTRACE=1 cargo run"
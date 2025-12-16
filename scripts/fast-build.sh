#!/usr/bin/env bash
# Fast build helper: build library only without GUI to speed iteration
set -euo pipefail

echo "Fast building library without GUI features..."
cargo build --no-default-features --lib

echo "Done. For running tests use: cargo test --no-default-features"
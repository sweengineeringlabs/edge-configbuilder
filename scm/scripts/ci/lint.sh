#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/scm"
cargo fmt --check
cargo clippy --all-targets -- -D warnings

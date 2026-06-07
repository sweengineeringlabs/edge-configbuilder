#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/scm"
cargo test
cargo test --features test-utils

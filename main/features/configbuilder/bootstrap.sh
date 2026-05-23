#!/usr/bin/env bash
set -euo pipefail
echo "==> swe-edge-config: fetching dependencies"
cargo fetch --locked
echo "Bootstrap complete."

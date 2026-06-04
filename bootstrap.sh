#!/usr/bin/env bash
# edge-configbuilder bootstrap — installs git hooks and fetches dependencies.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "==> Installing git hooks"
git -C "$REPO_ROOT" config core.hooksPath scripts/hooks
echo "    core.hooksPath -> scripts/hooks (pre-commit, commit-msg)"

echo "==> Fetching dependencies"
(cd "$REPO_ROOT" && cargo fetch --locked)

echo "Bootstrap complete."

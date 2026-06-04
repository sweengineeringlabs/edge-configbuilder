#!/usr/bin/env pwsh
# edge-configbuilder bootstrap — installs git hooks and fetches dependencies.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$RepoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "==> Installing git hooks"
git -C $RepoRoot config core.hooksPath scripts/hooks
Write-Host "    core.hooksPath -> scripts/hooks (pre-commit, commit-msg)"

Write-Host "==> Fetching dependencies"
Push-Location $RepoRoot
cargo fetch --locked
Pop-Location

Write-Host "Bootstrap complete."

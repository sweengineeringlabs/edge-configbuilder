#!/usr/bin/env pwsh
# edge-configbuilder bootstrap — installs git hooks and fetches dependencies.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$ScmRoot  = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Split-Path -Parent $ScmRoot

Write-Host "==> Installing git hooks"
git -C $RepoRoot config core.hooksPath scm/scripts/hooks
Write-Host "    core.hooksPath -> scm/scripts/hooks (pre-commit, commit-msg)"

Write-Host "==> Fetching dependencies"
Push-Location $ScmRoot
cargo fetch --locked
Pop-Location

Write-Host "Bootstrap complete."

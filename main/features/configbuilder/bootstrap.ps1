#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Write-Host "==> swe-edge-config: fetching dependencies"
cargo fetch --locked
Write-Host "Bootstrap complete."

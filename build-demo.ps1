# build-demo.ps1
# Build a demo crate to WASM and copy the outputs to shell/static/demos/<name>/
#
# Usage:
#   .\build-demo.ps1 pong-76
#   .\build-demo.ps1 maze-80
#   .\build-demo.ps1 jump-feel
#
# Prerequisites are checked and installed automatically.

param(
    [Parameter(Mandatory=$true)]
    [string]$DemoName
)

$ErrorActionPreference = "Stop"

# Paths
$CrateName  = $DemoName -replace "-", "_"          # pong-76 → pong_76
$CratePath  = "crates/ludeme-demos/$DemoName"
$TargetWasm = "target/wasm32-unknown-unknown/release/$CrateName.wasm"
$OutDir     = "shell/static/demos/$DemoName"

Write-Host ""
Write-Host "⬡ Ludeme WASM Build" -ForegroundColor Cyan
Write-Host "  Demo:   $DemoName"
Write-Host "  Crate:  $CratePath"
Write-Host "  Output: $OutDir"
Write-Host ""

# ── Prerequisite checks ─────────────────────────────────────────────

# 1a. Ensure wasm32-unknown-unknown target is installed
$targets = rustup target list --installed 2>&1
if ($targets -notcontains "wasm32-unknown-unknown") {
    Write-Host "→ Installing wasm32-unknown-unknown target..." -ForegroundColor Yellow
    rustup target add wasm32-unknown-unknown
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✗ Failed to add wasm32-unknown-unknown target" -ForegroundColor Red
        exit $LASTEXITCODE
    }
    Write-Host "  ✓ Target installed" -ForegroundColor Green
} else {
    Write-Host "  ✓ wasm32-unknown-unknown target found" -ForegroundColor DarkGray
}

# 1b. Ensure wasm-bindgen-cli is installed
$wbCmd = Get-Command wasm-bindgen -ErrorAction SilentlyContinue
if (-not $wbCmd) {
    Write-Host "→ Installing wasm-bindgen-cli (first time only)..." -ForegroundColor Yellow
    cargo install wasm-bindgen-cli
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✗ Failed to install wasm-bindgen-cli" -ForegroundColor Red
        exit $LASTEXITCODE
    }
    Write-Host "  ✓ wasm-bindgen-cli installed" -ForegroundColor Green
} else {
    Write-Host "  ✓ wasm-bindgen-cli found" -ForegroundColor DarkGray
}

Write-Host ""

# ── Build ────────────────────────────────────────────────────────────

# Check crate exists
if (-not (Test-Path $CratePath)) {
    Write-Host "✗ Crate not found at $CratePath" -ForegroundColor Red
    Write-Host "  Create the demo crate first." -ForegroundColor Red
    exit 1
}

# 2. Compile to WASM (release for smaller binary)
Write-Host "→ Compiling $DemoName to wasm32..." -ForegroundColor Yellow
cargo build -p $DemoName --target wasm32-unknown-unknown --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ cargo build failed" -ForegroundColor Red
    exit $LASTEXITCODE
}

# 3. Run wasm-bindgen to generate JS glue
Write-Host "→ Generating JS bindings with wasm-bindgen..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
wasm-bindgen $TargetWasm --out-dir $OutDir --target web --no-typescript
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ wasm-bindgen failed" -ForegroundColor Red
    exit $LASTEXITCODE
}

# 4. Print output files
Write-Host ""
Write-Host "  Build complete" -ForegroundColor Green
Write-Host "  Files in $OutDir :" -ForegroundColor Green
Get-ChildItem $OutDir | ForEach-Object {
    $name = $_.Name
    $kb   = [math]::Round($_.Length / 1024, 1)
    Write-Host "    $name  ($($kb)KB)"
}
Write-Host ""
Write-Host "  Shell path: /demos/$DemoName/$($CrateName).js" -ForegroundColor Cyan
Write-Host "  wasm_path:  /demos/$DemoName/$($CrateName).wasm" -ForegroundColor Cyan
Write-Host ""

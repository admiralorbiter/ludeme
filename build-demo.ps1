# build-demo.ps1
# Build a demo crate to WASM and copy the outputs to shell/static/demos/<name>/
#
# Usage:
#   .\build-demo.ps1 pong-76
#   .\build-demo.ps1 maze-80
#   .\build-demo.ps1 jump-feel
#
# Prerequisites:
#   - wasm32-unknown-unknown target: rustup target add wasm32-unknown-unknown
#   - wasm-bindgen-cli:              cargo install wasm-bindgen-cli

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

# Check crate exists
if (-not (Test-Path $CratePath)) {
    Write-Host "✗ Crate not found at $CratePath" -ForegroundColor Red
    Write-Host "  Create the demo crate first." -ForegroundColor Red
    exit 1
}

# 1. Compile to WASM (release for smaller binary)
Write-Host "→ Compiling $DemoName to wasm32..." -ForegroundColor Yellow
cargo build -p $DemoName --target wasm32-unknown-unknown --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ cargo build failed" -ForegroundColor Red
    exit $LASTEXITCODE
}

# 2. Run wasm-bindgen to generate JS glue
Write-Host "→ Generating JS bindings with wasm-bindgen..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
wasm-bindgen $TargetWasm --out-dir $OutDir --target web --no-typescript
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ wasm-bindgen failed" -ForegroundColor Red
    exit $LASTEXITCODE
}

# 3. Print output files
Write-Host ""
Write-Host "✓ Build complete" -ForegroundColor Green
Write-Host "  Files in $OutDir :" -ForegroundColor Green
Get-ChildItem $OutDir | ForEach-Object {
    $size = [math]::Round($_.Length / 1024, 1)
    Write-Host "    $($_.Name)  (${size}KB)"
}
Write-Host ""
Write-Host "  Shell path will be: /demos/$DemoName/${CrateName}.js" -ForegroundColor Cyan
Write-Host "  Set wasm_path in +page.server.ts to: /demos/$DemoName/${CrateName}.wasm" -ForegroundColor Cyan
Write-Host ""

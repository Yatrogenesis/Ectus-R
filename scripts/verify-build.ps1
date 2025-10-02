# Ectus-R Build Verification Script
# Verifies that C++ Build Tools are installed and Rust can compile

Write-Host "=== Ectus-R Build Verification ===" -ForegroundColor Cyan
Write-Host ""

# Check Visual Studio link.exe (not Git's)
Write-Host "1. Checking for MSVC linker..." -ForegroundColor Yellow
$vsPath = "C:\Program Files\Microsoft Visual Studio\2022"
$linkPaths = Get-ChildItem -Path $vsPath -Recurse -Filter "link.exe" -ErrorAction SilentlyContinue | Where-Object { $_.DirectoryName -like "*VC\Tools\MSVC*" }

if ($linkPaths) {
    Write-Host "   ✓ MSVC linker found: $($linkPaths[0].FullName)" -ForegroundColor Green
} else {
    Write-Host "   ✗ MSVC linker NOT found - C++ Build Tools not installed" -ForegroundColor Red
    Write-Host "   Install from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Yellow
    exit 1
}

# Check Rust toolchain
Write-Host ""
Write-Host "2. Checking Rust toolchain..." -ForegroundColor Yellow
$rustVersion = rustc --version
if ($rustVersion) {
    Write-Host "   ✓ $rustVersion" -ForegroundColor Green
} else {
    Write-Host "   ✗ Rust not found" -ForegroundColor Red
    exit 1
}

$rustTarget = rustup show | Select-String "active toolchain"
Write-Host "   ✓ $rustTarget" -ForegroundColor Green

# Test compilation
Write-Host ""
Write-Host "3. Testing Rust compilation..." -ForegroundColor Yellow
Write-Host "   This may take a few minutes..." -ForegroundColor Gray

$env:RUSTFLAGS = ""
cd $PSScriptRoot\..\

$buildResult = cargo build --release 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✓ Compilation SUCCESSFUL!" -ForegroundColor Green

    # Check binary
    $binaryPath = "target\release\aion-web-api.exe"
    if (Test-Path $binaryPath) {
        $binarySize = (Get-Item $binaryPath).Length / 1MB
        Write-Host "   ✓ Binary created: $binaryPath ($([math]::Round($binarySize, 2)) MB)" -ForegroundColor Green
    }
} else {
    Write-Host "   ✗ Compilation FAILED" -ForegroundColor Red
    Write-Host $buildResult -ForegroundColor Red
    exit 1
}

# Success
Write-Host ""
Write-Host "=== BUILD VERIFICATION PASSED ===" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Run backend: cargo run --bin aion-web-api" -ForegroundColor White
Write-Host "  2. Backend will start on: http://localhost:8080" -ForegroundColor White
Write-Host "  3. Frontend already running on: http://localhost:5173" -ForegroundColor White
Write-Host ""

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$editorPath = Join-Path $repoRoot "vendor/marktext"
$gitModulesPath = Join-Path $repoRoot ".gitmodules"

if (-not (Test-Path $gitModulesPath)) {
    throw "Could not find .gitmodules. Run this script from a Khwarizmi Studio checkout."
}

if (-not (Test-Path (Join-Path $editorPath "package.json"))) {
    Write-Host "Initializing the MarkText submodule..."
    git -C $repoRoot submodule update --init --recursive vendor/marktext
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to initialize vendor/marktext. Check your Git/network access."
    }
}

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    throw "Node.js is required. Install Node.js 22.12.0 or newer, then reopen PowerShell."
}

if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
    throw "pnpm 10 or newer is required. Enable Corepack or install pnpm, then reopen PowerShell."
}

& (Join-Path $repoRoot "scripts/patch-marktext.ps1")
if ($LASTEXITCODE -ne 0) { throw "Failed to apply Khwarizmi branding." }

Push-Location $editorPath
try {
    if (-not (Test-Path "node_modules")) {
        Write-Host "Installing MarkText dependencies (first run only)..."
        pnpm install
        if ($LASTEXITCODE -ne 0) {
            throw "Dependency installation failed."
        }
    }

    Write-Host "Starting Khwarizmi Studio (based on the MarkText editor engine)."
    pnpm run dev
    if ($LASTEXITCODE -ne 0) {
        throw "MarkText exited with an error."
    }
}
finally {
    Pop-Location
}

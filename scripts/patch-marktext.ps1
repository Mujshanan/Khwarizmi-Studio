$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$editorRoot = Join-Path $repoRoot "vendor/marktext"

if (-not (Test-Path (Join-Path $editorRoot "packages/desktop/electron-builder.yml"))) {
    throw "MarkText submodule is not initialized. Run scripts/run-editor.ps1 first."
}

function Replace-Text([string]$Path, [string]$Old, [string]$New) {
    $fullPath = Join-Path $editorRoot $Path
    $text = [System.IO.File]::ReadAllText($fullPath)
    if ($text.Contains($Old)) {
        $text = $text.Replace($Old, $New)
        [System.IO.File]::WriteAllText($fullPath, $text, [System.Text.UTF8Encoding]::new($false))
    } elseif (-not $text.Contains($New)) {
        throw "Expected branding text was not found in $Path. Upstream source may have changed."
    }
}

# Keep this patch small and explicit. It is applied after checkout and is not
# written into the pinned upstream submodule commit.
Replace-Text "packages/desktop/electron-builder.yml" "appId: com.github.marktext.marktext" "appId: com.khwarizmi.studio"
Replace-Text "packages/desktop/electron-builder.yml" "productName: MarkText" "productName: Khwarizmi Studio"
Replace-Text "packages/desktop/electron-builder.yml" "marktext-win-" "khwarizmi-studio-win-"
Replace-Text "packages/desktop/electron-builder.yml" "marktext-mac-" "khwarizmi-studio-mac-"
Replace-Text "packages/desktop/electron-builder.yml" "marktext-linux-" "khwarizmi-studio-linux-"
Replace-Text "packages/desktop/electron-builder.yml" "executableName: marktext" "executableName: khwarizmi-studio"
Replace-Text "packages/desktop/electron-builder.yml" "StartupWMClass: 'marktext'" "StartupWMClass: 'khwarizmi-studio'"
Replace-Text "packages/desktop/electron-builder.yml" "Keywords: 'marktext;'" "Keywords: 'khwarizmi-studio; '"

Replace-Text "packages/desktop/src/main/index.ts" "productName: 'MarkText'" "productName: 'Khwarizmi Studio'"
Replace-Text "packages/desktop/src/main/index.ts" "electronApp.setAppUserModelId('com.electron.marktext')" "electronApp.setAppUserModelId('com.khwarizmi.studio')"
Replace-Text "packages/desktop/src/renderer/index.html" "<title>MarkText</title>" "<title>Khwarizmi Studio</title>"

Write-Host "Applied Khwarizmi Studio packaging and startup branding."
Write-Host "The editor internals and many UI strings are still upstream MarkText."

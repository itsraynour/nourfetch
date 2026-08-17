# nourfetch - PowerShell uninstaller for Windows
# Usage: irm https://raw.githubusercontent.com/itsraynour/nourfetch/main/uninstall.ps1 | iex

$ErrorActionPreference = "SilentlyContinue"

Write-Host "Uninstalling nourfetch..." -ForegroundColor Cyan

$removedAny = $false

$installDir = "$env:LOCALAPPDATA\Microsoft\WindowsApps"
$targetExe = "$installDir\nourfetch.exe"
if (Test-Path $targetExe) {
    Remove-Item -Path $targetExe -Force -ErrorAction SilentlyContinue
    Write-Host "  Removed executable: $targetExe" -ForegroundColor Green
    $removedAny = $true
}

$cargoBin = "$env:USERPROFILE\.cargo\bin\nourfetch.exe"
if (Test-Path $cargoBin) {
    Remove-Item -Path $cargoBin -Force -ErrorAction SilentlyContinue
    Write-Host "  Removed cargo executable: $cargoBin" -ForegroundColor Green
    $removedAny = $true
}

$configDir1 = "$env:APPDATA\nourfetch"
if (Test-Path $configDir1) {
    Remove-Item -Path $configDir1 -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "  Removed directory: $configDir1" -ForegroundColor Green
    $removedAny = $true
}

$configDir2 = "$env:USERPROFILE\.config\nourfetch"
if (Test-Path $configDir2) {
    Remove-Item -Path $configDir2 -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "  Removed directory: $configDir2" -ForegroundColor Green
    $removedAny = $true
}

Write-Host ""
if ($removedAny) {
    Write-Host "nourfetch has been completely uninstalled." -ForegroundColor Green
} else {
    Write-Host "No installed files or configurations found." -ForegroundColor Yellow
}
Write-Host ""

# nourfetch - PowerShell installer for Windows
# Usage: irm https://raw.githubusercontent.com/itsraynour/nourfetch/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "Installing nourfetch for Windows..." -ForegroundColor Cyan

$installDir = "$env:LOCALAPPDATA\Microsoft\WindowsApps"
$targetExe = "$installDir\nourfetch.exe"

$localExe = "$PSScriptRoot\target\release\nourfetch.exe"
if (-not (Test-Path $localExe)) {
    $localExe = "$PSScriptRoot\nourfetch.exe"
}

if (Test-Path $localExe) {
    Copy-Item $localExe -Destination $targetExe -Force
    Write-Host "Installed local binary to $installDir" -ForegroundColor Green
} else {
    $repo = "itsraynour/nourfetch"
    $downloadUrl = "https://github.com/$repo/releases/latest/download/nourfetch-windows-x86_64.exe"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $targetExe -UseBasicParsing
        Write-Host "Downloaded latest binary to $targetExe" -ForegroundColor Green
    } catch {
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            Write-Host "Building from source..." -ForegroundColor Yellow
            cargo build --release
            Copy-Item "$PSScriptRoot\target\release\nourfetch.exe" -Destination $targetExe -Force
        } else {
            Write-Error "Failed to install nourfetch. Please check your network connection or build from source using cargo."
        }
    }
}

Write-Host "nourfetch installed successfully." -ForegroundColor Green
Write-Host "Run 'nourfetch' in PowerShell or Command Prompt." -ForegroundColor Cyan

& "$targetExe"

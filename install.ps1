# nourfetch - PowerShell installer for Windows
# Usage: irm https://raw.githubusercontent.com/itsraynour/nourfetch/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

Write-Host "Installing nourfetch for Windows..." -ForegroundColor Cyan

$repo = "itsraynour/nourfetch"
$installDir = "$env:LOCALAPPDATA\Microsoft\WindowsApps"

if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

$targetExe = "$installDir\nourfetch.exe"
$installed = $false

if ($PSScriptRoot) {
    $localExe = "$PSScriptRoot\target\release\nourfetch.exe"
    if (-not (Test-Path $localExe)) {
        $localExe = "$PSScriptRoot\nourfetch-windows-x86_64.exe"
    }
    if (-not (Test-Path $localExe)) {
        $localExe = "$PSScriptRoot\nourfetch.exe"
    }
    
    if (Test-Path $localExe) {
        Copy-Item -Path $localExe -Destination $targetExe -Force
        Write-Host "Installed local binary to $installDir" -ForegroundColor Green
        $installed = $true
    }
}

if (-not $installed) {
    $downloadUrl = "https://github.com/$repo/releases/latest/download/nourfetch-windows-x86_64.exe"
    Write-Host "Downloading latest release from GitHub..." -ForegroundColor Cyan
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $targetExe -UseBasicParsing
        Write-Host "Downloaded binary to $targetExe" -ForegroundColor Green
        $installed = $true
    } catch {
        if (Get-Command cargo -ErrorAction SilentlyContinue) {
            Write-Host "Building from source using Cargo..." -ForegroundColor Yellow
            cargo install --git "https://github.com/$repo.git"
            $installed = $true
        } else {
            Write-Error "Failed to install nourfetch. Please check your internet connection or download manually from: https://github.com/$repo/releases"
            return
        }
    }
}

$userPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
$machinePath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine)
$allPath = "$userPath;$machinePath;$env:PATH"

if ($allPath -notlike "*$installDir*") {
    $newUserPath = if ($userPath) { "$userPath;$installDir" } else { $installDir }
    [Environment]::SetEnvironmentVariable("Path", $newUserPath, [EnvironmentVariableTarget]::User)
    $env:PATH = "$env:PATH;$installDir"
}

Write-Host ""
Write-Host "nourfetch installed successfully." -ForegroundColor Green
Write-Host "Run 'nourfetch' in PowerShell or Command Prompt." -ForegroundColor Cyan
Write-Host ""

if (Test-Path $targetExe) {
    & "$targetExe"
}



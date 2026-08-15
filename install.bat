@echo off
setlocal enabledelayedexpansion

set "REPO=itsraynour/nourfetch"
set "DEST_DIR=%LOCALAPPDATA%\Microsoft\WindowsApps"
set "TARGET_EXE=%DEST_DIR%\nourfetch.exe"

if not exist "%DEST_DIR%" (
    mkdir "%DEST_DIR%" 2>nul
)

set "INSTALLED=0"

if exist "%~dp0target\release\nourfetch.exe" (
    copy /Y "%~dp0target\release\nourfetch.exe" "%TARGET_EXE%" >nul
    set "INSTALLED=1"
) else if exist "%~dp0nourfetch-windows-x86_64.exe" (
    copy /Y "%~dp0nourfetch-windows-x86_64.exe" "%TARGET_EXE%" >nul
    set "INSTALLED=1"
) else if exist "%~dp0nourfetch.exe" (
    if /I not "%~dp0nourfetch.exe"=="%TARGET_EXE%" (
        copy /Y "%~dp0nourfetch.exe" "%TARGET_EXE%" >nul
        set "INSTALLED=1"
    )
)

if "!INSTALLED!"=="0" (
    echo Downloading nourfetch from GitHub releases...
    curl -fsSL "https://github.com/%REPO%/releases/latest/download/nourfetch-windows-x86_64.exe" -o "%TARGET_EXE%"
    if exist "%TARGET_EXE%" (
        set "INSTALLED=1"
    )
)

if "!INSTALLED!"=="0" (
    where cargo >nul 2>nul
    if !ERRORLEVEL! EQU 0 (
        echo Building nourfetch from source...
        cargo build --release
        if exist "%~dp0target\release\nourfetch.exe" (
            copy /Y "%~dp0target\release\nourfetch.exe" "%TARGET_EXE%" >nul
            set "INSTALLED=1"
        )
    )
)

if "!INSTALLED!"=="0" (
    echo Error: Failed to install nourfetch.
    echo Please check your internet connection or download manually from:
    echo https://github.com/%REPO%/releases
    pause
    exit /b 1
)

echo nourfetch installed successfully.
echo Run 'nourfetch' from any terminal.
echo.

if exist "%TARGET_EXE%" (
    "%TARGET_EXE%"
)

echo.
pause



@echo off
setlocal

set "REPO=itsraynour/nourfetch"
set "DEST_DIR=%LOCALAPPDATA%\Microsoft\WindowsApps"
set "EXE_SRC=%~dp0target\release\nourfetch.exe"

if not exist "%EXE_SRC%" (
    set "EXE_SRC=%~dp0nourfetch.exe"
)

if not exist "%EXE_SRC%" (
    echo Downloading nourfetch from GitHub releases...
    curl -fsSL "https://github.com/%REPO%/releases/latest/download/nourfetch-windows-x86_64.exe" -o "%DEST_DIR%\nourfetch.exe" 2>nul
    if exist "%DEST_DIR%\nourfetch.exe" (
        set "EXE_SRC=%DEST_DIR%\nourfetch.exe"
    )
)

if not exist "%EXE_SRC%" (
    echo Building nourfetch from source...
    cargo build --release
    set "EXE_SRC=%~dp0target\release\nourfetch.exe"
)

if not exist "%EXE_SRC%" (
    echo Error: Could not install nourfetch.
    pause
    exit /b 1
)

if /I not "%EXE_SRC%"=="%DEST_DIR%\nourfetch.exe" (
    copy /Y "%EXE_SRC%" "%DEST_DIR%\nourfetch.exe" >nul
)

if %ERRORLEVEL% EQU 0 (
    echo nourfetch installed successfully.
    echo Run 'nourfetch' from any terminal.
    echo.
    "%DEST_DIR%\nourfetch.exe"
) else (
    echo Error: Installation failed.
)

echo.
pause

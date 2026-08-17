@echo off
setlocal enabledelayedexpansion

echo Uninstalling nourfetch...

set "REMOVED=0"

set "DEST_DIR=%LOCALAPPDATA%\Microsoft\WindowsApps"
set "TARGET_EXE=%DEST_DIR%\nourfetch.exe"

if exist "%TARGET_EXE%" (
    del /f /q "%TARGET_EXE%" >nul 2>&1
    echo   Removed %TARGET_EXE%
    set "REMOVED=1"
)

if exist "%USERPROFILE%\.cargo\bin\nourfetch.exe" (
    del /f /q "%USERPROFILE%\.cargo\bin\nourfetch.exe" >nul 2>&1
    echo   Removed %USERPROFILE%\.cargo\bin\nourfetch.exe
    set "REMOVED=1"
)

if exist "%APPDATA%\nourfetch" (
    rmdir /s /q "%APPDATA%\nourfetch" >nul 2>&1
    echo   Removed %APPDATA%\nourfetch
    set "REMOVED=1"
)

if exist "%USERPROFILE%\.config\nourfetch" (
    rmdir /s /q "%USERPROFILE%\.config\nourfetch" >nul 2>&1
    echo   Removed %USERPROFILE%\.config\nourfetch
    set "REMOVED=1"
)

echo.
if "!REMOVED!"=="1" (
    echo nourfetch has been completely uninstalled.
) else (
    echo No installed files or configurations found.
)
echo.
pause

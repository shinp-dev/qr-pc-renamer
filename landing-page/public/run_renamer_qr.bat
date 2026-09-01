@echo off
chcp 65001 > nul
cd /d "%~dp0"

:: 管理者権限チェック（権限がなければここで昇格を促す）
openfiles >nul 2>&1
if %errorlevel% neq 0 (
    echo 管理者権限で実行し直しています...
    powershell -NoProfile -ExecutionPolicy Bypass -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

set "EXE_PATH=%~dp0target\release\pc_renamer.exe"

echo ==============================================
echo 実行ユーザー : %USERNAME% (管理者権限)
echo 起動モード   : QRコードスキャン / 手動入力
echo ==============================================
echo.

if exist "%EXE_PATH%" (
    "%EXE_PATH%"
) else (
    echo 【エラー】EXEが見つかりません: %EXE_PATH%
)

pause

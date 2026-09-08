@echo off
setlocal

cd /d "%~dp0"

if not defined OMNISOLO_STANDALONE set "OMNISOLO_STANDALONE=true"
if not defined STANDALONE_MODE set "STANDALONE_MODE=true"
if not defined DATABASE_URL set "DATABASE_URL=sqlite://.omnisolo/omnisolo-standalone.db"
if not defined OMNISOLO_PORT set "OMNISOLO_PORT=18789"
if not defined OMNISOLO_GRPC_PORT set "OMNISOLO_GRPC_PORT=8081"
if not defined OMNISOLO_AGENT_ADDRESS set "OMNISOLO_AGENT_ADDRESS=127.0.0.1:50051"

if not exist ".omnisolo" mkdir ".omnisolo"

set "OMNISOLO_SQLITE_KEY_FILE=%CD%\.omnisolo\sqlite.key"
if not exist "%OMNISOLO_SQLITE_KEY_FILE%" (
  powershell -NoProfile -ExecutionPolicy Bypass -Command "$bytes = New-Object byte[] 32; [Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes); [Convert]::ToBase64String($bytes) | Set-Content -Encoding ASCII -NoNewline $env:OMNISOLO_SQLITE_KEY_FILE"
  icacls "%OMNISOLO_SQLITE_KEY_FILE%" /inheritance:r /grant "%USERNAME%:F" /c /q >nul 2>&1
)

if not defined OMNISOLO_SQLITE_KEY (
  for /f "usebackq delims=" %%K in ("%OMNISOLO_SQLITE_KEY_FILE%") do set "OMNISOLO_SQLITE_KEY=%%K"
)

echo Starting OmniSolo portable server...
echo URL: http://127.0.0.1:%OMNISOLO_PORT%/
echo Data: %CD%\.omnisolo
echo.

".\omnisolo-server.exe"
set "OMNISOLO_EXIT_CODE=%ERRORLEVEL%"

if not "%OMNISOLO_EXIT_CODE%"=="0" (
  echo.
  echo OmniSolo exited with code %OMNISOLO_EXIT_CODE%.
  pause
)

exit /b %OMNISOLO_EXIT_CODE%

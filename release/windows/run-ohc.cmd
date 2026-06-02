@echo off
setlocal

cd /d "%~dp0"

if not defined OHC_STANDALONE set "OHC_STANDALONE=true"
if not defined STANDALONE_MODE set "STANDALONE_MODE=true"
if not defined DATABASE_URL set "DATABASE_URL=sqlite://.ohc/ohc-standalone.db"
if not defined OHC_PORT set "OHC_PORT=18789"
if not defined OHC_GRPC_PORT set "OHC_GRPC_PORT=8081"
if not defined OHC_AGENT_ADDRESS set "OHC_AGENT_ADDRESS=127.0.0.1:50051"

if not exist ".ohc" mkdir ".ohc"

set "OHC_SQLITE_KEY_FILE=%CD%\.ohc\sqlite.key"
if not exist "%OHC_SQLITE_KEY_FILE%" (
  powershell -NoProfile -ExecutionPolicy Bypass -Command "$bytes = New-Object byte[] 32; [Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes); [Convert]::ToBase64String($bytes) | Set-Content -Encoding ASCII -NoNewline $env:OHC_SQLITE_KEY_FILE"
)

if not defined OHC_SQLITE_KEY (
  for /f "usebackq delims=" %%K in ("%OHC_SQLITE_KEY_FILE%") do set "OHC_SQLITE_KEY=%%K"
)

echo Starting OHC portable server...
echo URL: http://127.0.0.1:%OHC_PORT%/
echo Data: %CD%\.ohc
echo.

".\ohc-server.exe"
set "OHC_EXIT_CODE=%ERRORLEVEL%"

if not "%OHC_EXIT_CODE%"=="0" (
  echo.
  echo OHC exited with code %OHC_EXIT_CODE%.
  pause
)

exit /b %OHC_EXIT_CODE%

#!/bin/bash
set -e
echo "==============================================="
echo "   🚀 OHC Quick Start (Day One Onboarding)    "
echo "==============================================="
echo "[1/3] Verifying dependencies..."
command -v bazelisk >/dev/null 2>&1 || { echo "Bazelisk is required."; kill -INT $$; }
command -v go >/dev/null 2>&1 || { echo "Go is required."; kill -INT $$; }
echo "[2/3] Setting up environment..."
if [ ! -f .env ]; then
  echo "LOG_LEVEL=info" > .env
  echo "PORT=8080" >> .env
  echo "OHC_MULTITENANT=false" >> .env
  echo "OHC_HEADLESS=false" >> .env
  echo "OHC_SOURCE_MODE=standalone" >> .env
  chmod 0600 .env
fi
echo "[3/3] Launching local backend..."
export OHC_MULTITENANT=false
export OHC_SOURCE_MODE=standalone
bazelisk run //srcs/server:ohc &
SERVER_PID=$!
echo "Server started with PID $SERVER_PID. To stop, run: kill $SERVER_PID"

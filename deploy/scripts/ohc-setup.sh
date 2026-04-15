#!/bin/bash
# OHC Hybrid Developer Setup Script

set -e

echo "==============================================="
echo "   🚀 OHC Hybrid Agentic OS Developer Setup    "
echo "==============================================="

# Check requirements
command -v bazelisk >/dev/null 2>&1 || { echo "Bazelisk is required but not installed. Aborting."; return 1 2>/dev/null || exit 1; }
command -v docker >/dev/null 2>&1 || { echo "Docker is required but not installed. Aborting."; return 1 2>/dev/null || exit 1; }

echo "[1/4] Checking environment configuration..."
if [ ! -f .env ]; then
  echo "Creating default .env file..."
  cat << 'ENV' > .env
# Default Local Config
LOG_LEVEL=info
PORT=8080
OHC_MULTITENANT=false
OHC_HEADLESS=false
OHC_SOURCE_MODE=standalone
OHC_RUNTIME_DIR=.ohc/runtime
OHC_MEMORY_DIR=.ohc/runtime/memory
OHC_STATUS_DIR=.ohc/runtime/status
ENV
  chmod 0600 .env
fi

echo "[2/4] Verifying Standalone Mode..."
export OHC_MULTITENANT=false
export OHC_HEADLESS=false
export OHC_SOURCE_MODE=standalone
bazelisk test //...

echo "[3/4] Verifying Cloud Mode..."
export OHC_MULTITENANT=true
export OHC_HEADLESS=false
export OHC_SOURCE_MODE=cloud
bazelisk test //...

echo "[4/4] Generating Local Memory Log..."
RUNTIME_DIR="${OHC_RUNTIME_DIR:-.ohc/runtime}"
MEMORY_DIR="${OHC_MEMORY_DIR:-${RUNTIME_DIR}/memory}"
STATUS_DIR="${OHC_STATUS_DIR:-${RUNTIME_DIR}/status}"
mkdir -p "${MEMORY_DIR}" "${STATUS_DIR}"
TIMESTAMP=$(date +%s)

cat << MEM > "${MEMORY_DIR}/setup-${TIMESTAMP}.yml"
type: memory
metadata:
  role: Developer Setup
  timestamp: ${TIMESTAMP}
observations:
  - Developer executed ohc-setup.sh
actions_taken:
  - Verified local environment
  - Built Standalone and Cloud targets
resolution: Developer environment successfully initialized.
MEM

cat << STAT > "${STATUS_DIR}/${TIMESTAMP}.yml"
type: status
metadata:
  role: Developer Setup
  timestamp: ${TIMESTAMP}
health: ok
observations:
  - ohc-setup.sh completed successfully.
STAT

echo "==============================================="
echo "   ✅ Setup Complete!                          "
echo "   Use 'source deploy/scripts/ohc-mode.sh' to switch contexts."
echo "==============================================="

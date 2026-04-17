#!/bin/bash
# OHC Hybrid Development Mode Switcher

MODE=$1
DEFAULT_RUNTIME_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/ohc/runtime"
DEFAULT_MEMORY_DIR="${DEFAULT_RUNTIME_DIR}/memory"

if [ -z "$MODE" ]; then
  echo "Usage: source ./ohc-mode.sh [cloud|standalone|headless]"
  return 1 2>/dev/null || exit 1
fi

echo "Switching OHC Development Environment to: $MODE"

case $MODE in
  cloud)
    export OHC_MULTITENANT=true
    export OHC_HEADLESS=false
    export OHC_SOURCE_MODE=cloud
    export LOG_FORMAT="json"
    export LOG_LEVEL="info"
    echo "Configured for Cloud-Native Multi-Tenant Mode."
    ;;
  standalone)
    export OHC_MULTITENANT=false
    export OHC_HEADLESS=false
    export OHC_SOURCE_MODE=standalone
    export GOMAXPROCS=2
    export OHC_STANDALONE=true
    export GOMEMLIMIT="256MiB"
    export GOGC=50
    export OHC_RUNTIME_DIR="${OHC_RUNTIME_DIR:-${DEFAULT_RUNTIME_DIR}}"
    export OHC_MEMORY_DIR="${OHC_MEMORY_DIR:-${DEFAULT_MEMORY_DIR}}"
    export OHC_STATUS_DIR="${OHC_STATUS_DIR:-${DEFAULT_RUNTIME_DIR}/status}"
    mkdir -p "${OHC_MEMORY_DIR}/auto/" "${OHC_MEMORY_DIR}/team/" "${OHC_STATUS_DIR}"
    export LOG_FORMAT="text"
    export LOG_LEVEL="info"
    if [ "$OHC_TELEMETRY_ENABLED" != "true" ]; then
      export OHC_TELEMETRY_ENABLED=false
    fi
    echo "Configured for Standalone Desktop Mode."
    ;;
  headless)
    export OHC_MULTITENANT=false
    export OHC_HEADLESS=true
    export OHC_SOURCE_MODE=cloud
    export LOG_FORMAT="json"
    export LOG_LEVEL="info"
    echo "Configured for Headless API Mode."
    ;;
  *)
    echo "Unknown mode: $MODE"
    echo "Valid modes: cloud, standalone, headless"
    return 1 2>/dev/null || exit 1
    ;;
esac

echo "Environment variables set. You can now run bazelisk commands."

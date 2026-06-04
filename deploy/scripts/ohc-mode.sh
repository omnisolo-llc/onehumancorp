#!/bin/bash
# OHC Hybrid Development Mode Switcher

MODE=$1

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
    export TOKIO_WORKER_THREADS=2
    export RAYON_NUM_THREADS=2
    export OHC_STANDALONE=true
    mkdir -p .ohc/memory/auto/ .ohc/memory/team/
    export LOG_FORMAT="json"
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

#!/bin/bash
# OmniSolo Hybrid Development Mode Switcher

MODE=$1

if [ -z "$MODE" ]; then
  echo "Usage: source ./omnisolo-mode.sh [cloud|standalone|headless]"
  return 1 2>/dev/null || exit 1
fi

echo "Switching OmniSolo Development Environment to: $MODE"

case $MODE in
  cloud)
    export OMNISOLO_MULTITENANT=true
    export OMNISOLO_HEADLESS=false
    export OMNISOLO_SOURCE_MODE=cloud
    export OMNISOLO_STANDALONE_MODE=false
    export LOG_FORMAT="json"
    export LOG_LEVEL="info"
    echo "Configured for Cloud-Native Multi-Tenant Mode."
    ;;
  standalone)
    export OMNISOLO_MULTITENANT=false
    export OMNISOLO_HEADLESS=false
    export OMNISOLO_SOURCE_MODE=standalone
    export OMNISOLO_STANDALONE_MODE=true
    export TOKIO_WORKER_THREADS=2
    export RAYON_NUM_THREADS=2
    mkdir -p .ohc/memory/auto/ .ohc/memory/team/
    export LOG_FORMAT="json"
    export LOG_LEVEL="info"
    if [ "$OMNISOLO_TELEMETRY_ENABLED" != "true" ]; then
      export OMNISOLO_TELEMETRY_ENABLED=false
    fi
    echo "Configured for Standalone Desktop Mode."
    ;;
  headless)
    export OMNISOLO_MULTITENANT=false
    export OMNISOLO_HEADLESS=true
    export OMNISOLO_SOURCE_MODE=cloud
    export OMNISOLO_STANDALONE_MODE=false
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

#!/bin/bash
# OHC Hybrid CLI tool for environment-switching
# Usage: ./ohc_hybrid_cli.sh [mode]
# Modes: cloud, standalone, headless

set -euo pipefail

MODE="${1:-}"

if [[ -z "$MODE" ]]; then
    echo "Usage: $0 [cloud|standalone|headless]"
    return 1 2>/dev/null || true
fi

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "--- OHC Hybrid CLI ---"

if [[ "$MODE" == "cloud" ]]; then
    echo "Switching to Cloud-Native Mode..."
    export OHC_MULTITENANT=true
    export OHC_HEADLESS=false
    cd "$PROJECT_ROOT/deploy" && docker compose up -d
    echo "Cloud-Native Mode running. Use 'docker compose logs -f' to view logs."
elif [[ "$MODE" == "standalone" ]]; then
    echo "Switching to Standalone Mode..."
    export OHC_MULTITENANT=false
    export OHC_HEADLESS=false
    echo "Starting OHC Desktop App (Standalone Mode)..."
    cd "$PROJECT_ROOT" && bazelisk run //:desktop
elif [[ "$MODE" == "headless" ]]; then
    echo "Switching to Headless Cloud API Mode..."
    export OHC_MULTITENANT=true
    export OHC_HEADLESS=true
    cd "$PROJECT_ROOT/deploy" && docker compose up -d server ohc-core postgres redis
    echo "Headless Cloud API Mode running."
else
    echo "Invalid mode: $MODE"
    echo "Usage: $0 [cloud|standalone|headless]"
    return 1 2>/dev/null || true
fi

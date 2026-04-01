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
echo "==============================================="
echo "   🚀 OHC Hybrid CLI                           "
echo "==============================================="
echo "Mode selected: $MODE"

# Create a localized environment variable context file
ENV_FILE="$PROJECT_ROOT/.env"
echo "Generating $ENV_FILE for $MODE target..."

if [[ "$MODE" == "cloud" ]]; then
    echo "Switching to Cloud-Native Mode..."
    cat << 'ENV' > "$ENV_FILE"
OHC_MULTITENANT=true
OHC_HEADLESS=false
DATABASE_URL=postgres://ohc:ohc@localhost:5432/ohc?sslmode=disable
REDIS_URL=redis://localhost:6379/0
ENV
    echo "Syncing Bazel targets for Cloud Mode..."
    export PATH="$PATH:$(go env GOPATH)/bin"
    cd "$PROJECT_ROOT" && bazelisk run //:gazelle

    cd "$PROJECT_ROOT/deploy" && docker compose up -d postgres redis server
    echo "Cloud-Native Mode running. Use 'docker compose logs -f' to view logs."

elif [[ "$MODE" == "standalone" ]]; then
    echo "Switching to Standalone Mode..."
    cat << 'ENV' > "$ENV_FILE"
OHC_MULTITENANT=false
OHC_HEADLESS=false
OHC_STANDALONE=true
ENV
    echo "Syncing Bazel targets for Standalone Mode..."
    export PATH="$PATH:$(go env GOPATH)/bin"
    cd "$PROJECT_ROOT" && bazelisk run //:gazelle

    echo "Starting OHC Desktop App (Standalone Mode)..."
    cd "$PROJECT_ROOT" && bazelisk run //:desktop --define=OHC_MODE=standalone

elif [[ "$MODE" == "headless" ]]; then
    echo "Switching to Headless Cloud API Mode..."
    cat << 'ENV' > "$ENV_FILE"
OHC_MULTITENANT=true
OHC_HEADLESS=true
DATABASE_URL=postgres://ohc:ohc@localhost:5432/ohc?sslmode=disable
REDIS_URL=redis://localhost:6379/0
ENV
    echo "Syncing Bazel targets for Headless Mode..."
    export PATH="$PATH:$(go env GOPATH)/bin"
    cd "$PROJECT_ROOT" && bazelisk run //:gazelle

    cd "$PROJECT_ROOT/deploy" && docker compose up -d server ohc-core postgres redis
    echo "Headless Cloud API Mode running."

else
    echo "Invalid mode: $MODE"
    echo "Usage: $0 [cloud|standalone|headless]"
    return 1 2>/dev/null || true
fi

echo "Environment successfully aligned to $MODE."

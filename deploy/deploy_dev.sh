#!/bin/bash
# Script to rebuild and start Docker Compose services locally via Bazel.
# This script is called via 'bazel run //:deploy_dev'.

set -e

# Support both 'docker compose' and 'docker-compose'
DOCKER_COMPOSE_CMD="docker compose"
if ! command -v docker &> /dev/null || ! docker compose version &> /dev/null; then
  if command -v docker-compose &> /dev/null; then
    DOCKER_COMPOSE_CMD="docker-compose"
  else
    echo "Error: 'docker compose' or 'docker-compose' not found. Please install Docker Compose."
    exit 1
  fi
fi

# Determine the project root.
if [[ -n "$BUILD_WORKSPACE_DIRECTORY" ]]; then
  PROJECT_ROOT="$BUILD_WORKSPACE_DIRECTORY"
else
  PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi

echo "--- Setting up Local Kubernetes Context ---"
if [[ -f "$PROJECT_ROOT/deploy/setup_k8s.sh" ]]; then
  bash "$PROJECT_ROOT/deploy/setup_k8s.sh" || true
else
  # Fallback for bazel runfiles
  SETUP_SCRIPT=$(find . -name setup_k8s.sh | head -n 1)
  if [[ -n "$SETUP_SCRIPT" ]]; then
    bash "$SETUP_SCRIPT" || true
  fi
fi

echo "--- Loading Bazel-built images ---"

# The oci_load target `//deploy:server_load` provides an executable script to load the image.
# We need to find and execute it.
# It should be in the runfiles tree.
SERVER_LOAD_SCRIPT=$(find . -name "server_load" -type f -executable | head -n 1)

if [[ -n "$SERVER_LOAD_SCRIPT" && -x "$SERVER_LOAD_SCRIPT" ]]; then
  echo "Loading server image using $SERVER_LOAD_SCRIPT"
  "$SERVER_LOAD_SCRIPT"
else
  # Try running the load script via bazel if it's not in runfiles
  echo "server_load script not found in runfiles, falling back to trying from project root"
  cd "$PROJECT_ROOT"
  if [[ -f "bazel-bin/deploy/server_load" ]]; then
      "bazel-bin/deploy/server_load"
  else
      echo "Error: could not find server_load script. Make sure //deploy:server_load is built and included in data."
      exit 1
  fi
fi

cd "$PROJECT_ROOT"

echo "--- Starting services from $PROJECT_ROOT ---"
# Run docker-compose without --build because we just loaded the images.
$DOCKER_COMPOSE_CMD -f deploy/docker-compose.yml up "$@"

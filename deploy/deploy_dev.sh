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
# bazel run outputs the tarballs to fixed locations in the runfiles.
# We'll use the environment variable if available, or just check the current dir.
cd "$PROJECT_ROOT"

# Load server image
echo "--- Loading Bazel-built images ---"
if [[ -n "$BUILD_WORKSPACE_DIRECTORY" ]]; then
  cd "$BUILD_WORKSPACE_DIRECTORY"
fi

if [[ -n "$RUNFILES_DIR" ]]; then
  SERVER_LOAD=$(find -L "$RUNFILES_DIR" -name "server_load.sh" | head -n 1)
  if [[ -n "$SERVER_LOAD" ]]; then
     bash "$SERVER_LOAD"
  else
     echo "Could not find server_load.sh in runfiles."
     exit 1
  fi
elif [[ -f "bazel-bin/deploy/server_load/tarball.tar" ]]; then
  docker load -i bazel-bin/deploy/server_load/tarball.tar
else
  echo "Error: Please run via bazel run //:deploy_dev"
  exit 1
fi

echo "--- Starting services from $PROJECT_ROOT ---"
# Run docker-compose without --build because we just loaded the images.
$DOCKER_COMPOSE_CMD -f deploy/docker-compose.yml up "$@"

#!/usr/bin/env bash
# Build the native images and explicitly launch the local Compose stack.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
if docker compose version >/dev/null 2>&1; then
  compose=(docker compose)
elif command -v docker-compose >/dev/null 2>&1; then
  compose=(docker-compose)
else
  echo "Docker Compose is required" >&2
  exit 1
fi
# Compose startup must not change a Kubernetes context or create a cluster.
bash deploy/load_all_images
COMPOSE_ENV_FILE="${OMNISOLO_COMPOSE_STATE_DIR:-$ROOT/.omnisolo-compose}/compose.env"
if [[ ! -f "$COMPOSE_ENV_FILE" ]]; then
  bash deploy/scripts/prepare-compose-env.sh
fi
exec "${compose[@]}" --env-file "$COMPOSE_ENV_FILE" \
  -f deploy/docker-compose.yml -f deploy/docker-compose.override.yml up "$@"

#!/usr/bin/env bash
# Docker Compose smoke test for the OHC single-machine container stack.
#
# This test:
#   1. Loads the Bazel-built server image into Docker
#   2. Starts the default Compose stack: server + PostgreSQL + Valkey
#   3. Waits for the server health endpoint
#   4. Verifies DB-backed UI API endpoints return valid JSON
#   5. Tears the stack down on exit
#
# Prerequisites (on PATH): docker, curl
set -euo pipefail

PROJECT_NAME="ohc-docker-e2e-$$"
FAILED_COMMAND=""
FAILED_LINE=""

log() { echo "[docker-e2e] $*"; }

record_failure() {
  FAILED_LINE="$1"
  FAILED_COMMAND="$2"
}

trap 'record_failure "$LINENO" "$BASH_COMMAND"' ERR

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required tool '$1' not found on PATH" >&2
    exit 1
  fi
}

compose() {
  docker compose -p "${PROJECT_NAME}" -f "${COMPOSE_FILE}" "$@"
}

cleanup() {
  log "Stopping Docker Compose project ${PROJECT_NAME} ..."
  compose down -v --remove-orphans 2>/dev/null || true
}

dump_diagnostics() {
  if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    echo "::group::Docker E2E failure diagnostics"
  fi

  if [[ -n "${FAILED_COMMAND}" ]]; then
    echo "Failed command near line ${FAILED_LINE}: ${FAILED_COMMAND}" >&2
  fi

  docker version || true
  compose ps || true
  compose logs --no-color --tail=200 || true

  if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    echo "::endgroup::"
  fi
}

on_exit() {
  local status=$?
  if [[ ${status} -ne 0 ]]; then
    dump_diagnostics || true
  fi
  cleanup
  exit "${status}"
}
trap on_exit EXIT

for tool in docker curl; do
  require_tool "${tool}"
done

if [[ -n "${TEST_SRCDIR:-}" ]]; then
  workspace="${TEST_WORKSPACE:-mono}"
  REPO_ROOT="${TEST_SRCDIR}/${workspace}"
else
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
fi

COMPOSE_FILE="${REPO_ROOT}/deploy/docker-compose.yml"
SERVER_LOADER="${REPO_ROOT}/deploy/server_load.sh"
export OHC_DOCKER_SERVER_PORT="${OHC_DOCKER_SERVER_PORT:-127.0.0.1:0}"
export OHC_DOCKER_POSTGRES_PORT="${OHC_DOCKER_POSTGRES_PORT:-127.0.0.1:0}"
export OHC_DOCKER_VALKEY_PORT="${OHC_DOCKER_VALKEY_PORT:-127.0.0.1:0}"
export MINIMAX_API_KEY="${MINIMAX_API_KEY:-docker-compose-e2e-placeholder-key}"

if [[ ! -f "${SERVER_LOADER}" || ! -x "${SERVER_LOADER}" ]]; then
  SERVER_LOADER="$(find "${TEST_SRCDIR:-${REPO_ROOT}}" -name "server_load.sh" -type f -executable | head -1)"
fi

if [[ -z "${SERVER_LOADER}" || ! -x "${SERVER_LOADER}" ]]; then
  echo "error: could not find executable server_load.sh" >&2
  exit 1
fi

log "Repo root: ${REPO_ROOT}"
log "Compose file: ${COMPOSE_FILE}"
log "Loading server image: ${SERVER_LOADER}"
"${SERVER_LOADER}"

docker version
docker info

log "Starting Docker Compose stack ..."
export MINIMAX_API_KEY="${MINIMAX_API_KEY:-dummy_key_for_test}"
compose up -d postgres valkey server

SERVER_BINDING="$(compose port server 8080)"
SERVER_PORT="${SERVER_BINDING##*:}"
BASE_URL="http://127.0.0.1:${SERVER_PORT}"
log "Server binding: ${SERVER_BINDING}"

wait_for_server() {
  local max_attempts=45
  local attempt=0
  while (( attempt < max_attempts )); do
    if curl -fsS "${BASE_URL}/healthz" >/dev/null 2>&1; then
      return 0
    fi
    attempt=$((attempt + 1))
    sleep 2
  done
  echo "error: server did not become healthy after ${max_attempts} attempts" >&2
  return 1
}

wait_for_server

log "Verifying health and DB-backed UI endpoints ..."
curl -fsS "${BASE_URL}/healthz"
curl -fsS "${BASE_URL}/readyz"
curl -fsS "${BASE_URL}/api/ui/dashboard/metrics?tenant_id=default" | grep -q '"total_sales"'
curl -fsS "${BASE_URL}/api/ui/orders?tenant_id=default" | grep -q '^\['
curl -fsS "${BASE_URL}/api/ui/inbox/messages?tenant_id=default" | grep -q '^\['
curl -fsS "${BASE_URL}/api/ui/supply?tenant_id=default" | grep -q '"vendors"'

log "Docker Compose E2E checks passed."

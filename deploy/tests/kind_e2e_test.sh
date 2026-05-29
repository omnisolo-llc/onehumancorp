#!/usr/bin/env bash
# Kind cluster end-to-end smoke test for the OHC platform.
#
# This test:
#   1. Creates a temporary Kind cluster
#   2. Builds and loads Docker images into the cluster
#   3. Installs Redis and PostgreSQL for cloud/web mode
#   4. Installs the OHC application chart in cloud/web mode
#   5. Runs REST API smoke tests
#   6. Installs the OHC application chart in standalone/desktop mode
#   7. Runs the same REST API smoke tests against SQLite-backed standalone mode
#   8. Cleans up the cluster on exit
#
# Prerequisites (on PATH):
#   kind, helm, kubectl, docker, curl
set -euo pipefail

CLUSTER_NAME="ohc-e2e-$$"
NAMESPACE="ohc-e2e"
CLOUD_RELEASE_NAME="ohc-cloud"
STANDALONE_RELEASE_NAME="ohc-standalone"

log() { echo "[kind-e2e] $*"; }

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required tool '$1' not found on PATH" >&2
    exit 1
  fi
}

cleanup() {
  if [[ -n "${PF_PID:-}" ]]; then
    kill "${PF_PID}" 2>/dev/null || true
  fi
  log "Deleting Kind cluster ${CLUSTER_NAME} ..."
  kind delete cluster --name "${CLUSTER_NAME}" 2>/dev/null || true
}

dump_diagnostics() {
  log "Collecting Kubernetes diagnostics after failure ..."
  kubectl get pods --namespace "${NAMESPACE}" -o wide 2>/dev/null || true
  kubectl describe pods --namespace "${NAMESPACE}" 2>/dev/null || true
  kubectl logs --namespace "${NAMESPACE}" --all-containers --tail=100 -l "app=${CLOUD_RELEASE_NAME}-backend" 2>/dev/null || true
  kubectl logs --namespace "${NAMESPACE}" --all-containers --tail=100 -l "app=${STANDALONE_RELEASE_NAME}-backend" 2>/dev/null || true
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

# ── Prerequisites ──────────────────────────────────────────────────────────────
for tool in kind helm kubectl docker curl; do
  require_tool "${tool}"
done

# ── Locate repo root (works both inside and outside Bazel sandbox) ────────────
if [[ -n "${TEST_SRCDIR:-}" ]]; then
  workspace="${TEST_WORKSPACE:-mono}"
  REPO_ROOT="${TEST_SRCDIR}/${workspace}"
else
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
fi

log "Repo root: ${REPO_ROOT}"

CHART_DIR="${TEST_TMPDIR:-/tmp}/ohc-chart-$$"
rm -rf "${CHART_DIR}"
cp -RL "${REPO_ROOT}/deploy/helm/ohc" "${CHART_DIR}"
chmod -R u+w "${CHART_DIR}"

COMMON_HELM_SMOKE_ARGS=(
  --set backend.replicas=1
  --set backend.autoscaling.enabled=false
  --set backend.vpa.enabled=false
  --set backend.resources.requests.cpu=100m
  --set backend.resources.requests.memory=128Mi
  --set backend.resources.limits.cpu=500m
  --set backend.resources.limits.memory=512Mi
  --set redis.enabled=false
  --set cnpg.enabled=false
  --set ohcCore.enabled=false
  --set chatwoot.enabled=false
  --set powersync.enabled=false
  --set kube-prometheus-stack.enabled=false
  --set fluentBit.enabled=false
  --set resourceQuota.enabled=false
)

CLOUD_HELM_SMOKE_ARGS=(
  "${COMMON_HELM_SMOKE_ARGS[@]}"
  --set multiTenant.enabled=true
  --set-string backend.env.DATABASE_URL=postgres://ohc:ohc@postgres:5432/ohc
  --set-string backend.env.REDIS_URL=redis://redis-master:6379
  --set-string backend.env.REDIS_ADDR=redis-master:6379
  --set-string backend.env.STANDALONE_MODE=false
  --set-string backend.env.JWT_SECRET=kind-e2e-cloud-jwt-secret-at-least-32-bytes
)

STANDALONE_HELM_SMOKE_ARGS=(
  "${COMMON_HELM_SMOKE_ARGS[@]}"
  --set multiTenant.enabled=false
  --set-string backend.env.DATABASE_URL=sqlite:///tmp/ohc-standalone/standalone.db
  --set-string backend.env.OHC_SQLITE_KEY=kind-e2e-standalone-sqlite-key
  --set-string backend.env.OHC_STANDALONE=true
  --set-string backend.env.OHC_TELEMETRY_ENABLED=false
  --set-string backend.env.STANDALONE_MODE=true
)

# ── Create Kind cluster ────────────────────────────────────────────────────────
log "Creating Kind cluster '${CLUSTER_NAME}' ..."

# Set KUBECONFIG to a temporary file BEFORE creating the cluster to avoid
# trying to lock the user's read-only default kubeconfig in the sandbox.
export KUBECONFIG="${TEST_TMPDIR:-/tmp}/kind-kubeconfig-$$"
touch "${KUBECONFIG}"

kind create cluster --name "${CLUSTER_NAME}" --wait 120s

log "Waiting for cluster nodes ..."
kubectl wait --for=condition=Ready node --all --timeout=120s

# ── Locating Images ────────────────────────────────────────────────────────────
# If running under Bazel, we use the pre-built image loaders.
# In a manual run, we fallback to docker build (for dev convenience).
if [[ -n "${TEST_SRCDIR:-}" ]]; then
  log "Bazel environment detected. Loading images from runfiles..."
  SERVER_LOADER="${REPO_ROOT}/deploy/server_load.sh"

  if [[ ! -f "${SERVER_LOADER}" || ! -x "${SERVER_LOADER}" ]]; then
    SERVER_LOADER="$(find "${TEST_SRCDIR}" -name "server_load.sh" -type f -executable | head -1)"
  fi

  if [[ -z "${SERVER_LOADER}" || ! -x "${SERVER_LOADER}" ]]; then
    echo "error: could not find executable server_load.sh in Bazel runfiles" >&2
    exit 1
  fi

  log "Executing server loader: ${SERVER_LOADER}"
  "${SERVER_LOADER}"
  docker tag onehumancorp/server:latest onehumancorp/server:e2e
else
  require_tool bazelisk
  log "Manual run detected. Building server image via Bazel..."
  bazelisk run //deploy:server_load
  docker tag onehumancorp/server:latest onehumancorp/server:e2e
fi

# ── Add Helm repos ─────────────────────────────────────────────────────────────
log "Adding Bitnami Helm repo ..."
helm repo add bitnami https://charts.bitnami.com/bitnami 2>/dev/null || true
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts 2>/dev/null || true
helm repo update bitnami prometheus-community 2>/dev/null || true

log "Building chart dependencies ..."
helm dependency build "${CHART_DIR}" --skip-refresh

wait_for_backend() {
  local backend_url="$1"
  local max_attempts=30
  local attempt=0
  while (( attempt < max_attempts )); do
    if curl -sf "${backend_url}/healthz" >/dev/null 2>&1; then
      return 0
    fi
    (( attempt++ ))
    sleep 2
  done
  echo "error: backend did not become healthy after ${max_attempts} attempts" >&2
  return 1
}

stop_port_forward() {
  if [[ -n "${PF_PID:-}" ]]; then
    kill "${PF_PID}" 2>/dev/null || true
    wait "${PF_PID}" 2>/dev/null || true
    PF_PID=""
  fi
}

install_ohc_release() {
  local release_name="$1"
  local mode_name="$2"
  shift 2

  log "Installing OHC Helm chart (${mode_name}) ..."
  helm upgrade --install "${release_name}" "${CHART_DIR}" \
    --namespace "${NAMESPACE}" \
    --set backend.image=onehumancorp/server:e2e \
    "$@"

  kubectl rollout status \
    --namespace "${NAMESPACE}" \
    "deployment/${release_name}-backend" \
    --timeout=90s

  kubectl wait pod \
    --namespace "${NAMESPACE}" \
    -l "app=${release_name}-backend" \
    --for=condition=Ready \
    --timeout=120s
}

run_rest_smoke_tests() {
  local release_name="$1"
  local mode_name="$2"
  local local_port="$3"
  local backend_url="http://127.0.0.1:${local_port}"

  log "Port-forwarding ${mode_name} backend service ..."
  kubectl port-forward \
    --namespace "${NAMESPACE}" \
    "svc/${release_name}-backend" \
    "${local_port}:8080" &
  PF_PID=$!

  sleep 3

  log "Waiting for ${mode_name} backend /healthz ..."
  wait_for_backend "${backend_url}"

  log "Running ${mode_name} REST smoke tests ..."

# --- health check ---
  response="$(curl -sf "${backend_url}/healthz")"
  [[ "${response}" == "ok" ]] || { echo "healthz failed: ${response}" >&2; exit 1; }
  log "  /healthz ✓"

  response="$(curl -sf "${backend_url}/readyz")"
  [[ "${response}" == "ok" ]] || { echo "readyz failed: ${response}" >&2; exit 1; }
  log "  /readyz ✓"

# --- seed demo data ---
  curl -sf -X POST "${backend_url}/api/dev/seed" \
    -H 'Content-Type: application/json' \
    -d '{"scenario":"launch-readiness"}' >/dev/null
  log "  /api/dev/seed ✓"

# --- dashboard ---
  dashboard="$(curl -sf "${backend_url}/api/dashboard")"
  echo "${dashboard}" | grep -q '"organization"' || { echo "dashboard missing 'organization'" >&2; exit 1; }
  log "  /api/dashboard ✓"

# --- agents list ---
  agents="$(curl -sf "${backend_url}/api/agents")"
  echo "${agents}" | grep -q '\[' || { echo "agents response not a JSON array" >&2; exit 1; }
  log "  /api/agents ✓"

# --- hire agent ---
  hire_response="$(curl -sf -X POST "${backend_url}/api/agents/hire" \
    -H 'Content-Type: application/json' \
    -d '{"name":"E2E Test Agent","role":"SOFTWARE_ENGINEER","model":"gpt-4o-mini"}')"
  echo "${hire_response}" | grep -q '"id"' || { echo "hire agent failed: ${hire_response}" >&2; exit 1; }
  log "  /api/agents/hire ✓"

# --- meetings ---
  meetings="$(curl -sf "${backend_url}/api/meetings")"
  echo "${meetings}" | grep -q '\[' || { echo "meetings response not a JSON array" >&2; exit 1; }
  log "  /api/meetings ✓"

# --- costs ---
  costs="$(curl -sf "${backend_url}/api/costs")"
  echo "${costs}" | grep -q '"totalCostUSD"' || { echo "costs missing totalCostUSD" >&2; exit 1; }
  log "  /api/costs ✓"

# --- approval flow ---
  approval_response="$(curl -sf -X POST "${backend_url}/api/approvals/request" \
    -H 'Content-Type: application/json' \
    -d '{"agentId":"swe-1","action":"deploy-to-production","reason":"E2E test","estimatedCostUsd":0.01,"riskLevel":"low"}')"
  echo "${approval_response}" | grep -q '"id"' || { echo "approval create failed: ${approval_response}" >&2; exit 1; }
  approval_id="$(echo "${approval_response}" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)"
  log "  /api/approvals/request ✓ (id=${approval_id})"

  curl -sf -X PUT "${backend_url}/api/approvals/decide" \
    -H 'Content-Type: application/json' \
    -d "{\"approvalId\":\"${approval_id}\",\"decision\":\"approve\",\"decidedBy\":\"e2e-test\"}" >/dev/null
  log "  /api/approvals/decide ✓"

# --- warm handoff ---
  handoff_response="$(curl -sf -X POST "${backend_url}/api/handoffs" \
    -H 'Content-Type: application/json' \
    -d '{"fromAgentId":"swe-1","toHumanRole":"MANAGER","intent":"need-review","failedAttempts":1,"currentState":"blocked"}')"
  echo "${handoff_response}" | grep -q '"id"' || { echo "handoff create failed: ${handoff_response}" >&2; exit 1; }
  log "  /api/handoffs ✓"

# --- billing costs ---
  costs2="$(curl -sf "${backend_url}/api/costs")"
  echo "${costs2}" | grep -q '"totalCostUSD"' || { echo "costs2 missing totalCostUSD" >&2; exit 1; }
  log "  /api/costs (post-hire) ✓"

# --- skill pack import ---
  skill_response="$(curl -sf -X POST "${backend_url}/api/skills/import" \
    -H 'Content-Type: application/json' \
    -d '{"name":"E2E Skill Pack","domain":"testing","description":"e2e","source":"custom","roles":[{"role":"SOFTWARE_ENGINEER","basePrompt":"e2e prompt"}]}')"
  echo "${skill_response}" | grep -q '"id"' || { echo "skill import failed: ${skill_response}" >&2; exit 1; }
  log "  /api/skills/import ✓"

# --- org snapshot ---
  snapshot_response="$(curl -sf -X POST "${backend_url}/api/snapshots/create" \
    -H 'Content-Type: application/json' \
    -d '{"label":"e2e-snapshot"}')"
  echo "${snapshot_response}" | grep -q '"id"' || { echo "snapshot create failed: ${snapshot_response}" >&2; exit 1; }
  log "  /api/snapshots/create ✓"

  stop_port_forward
  log "  ${mode_name} smoke ✓"
}

# ── Helm Verification ──────────────────────────────────────────────────────────
log "Verifying Helm chart (cloud/web mode) ..."
helm lint "${CHART_DIR}" "${CLOUD_HELM_SMOKE_ARGS[@]}"
helm template "${CLOUD_RELEASE_NAME}" "${CHART_DIR}" "${CLOUD_HELM_SMOKE_ARGS[@]}" > /dev/null

log "Verifying Helm chart (standalone/desktop mode) ..."
helm lint "${CHART_DIR}" "${STANDALONE_HELM_SMOKE_ARGS[@]}"
helm template "${STANDALONE_RELEASE_NAME}" "${CHART_DIR}" "${STANDALONE_HELM_SMOKE_ARGS[@]}" > /dev/null

log "Loading images into Kind cluster ..."
kind load docker-image onehumancorp/server:e2e --name "${CLUSTER_NAME}"

# ── Create namespace ───────────────────────────────────────────────────────────
kubectl create namespace "${NAMESPACE}" --dry-run=client -o yaml | kubectl apply -f -

# ── Install PostgreSQL for the cloud/web backend smoke test ───────────────────
log "Installing PostgreSQL ..."
kubectl run postgres \
  --namespace "${NAMESPACE}" \
  --image ankane/pgvector:v0.5.1 \
  --env POSTGRES_USER=ohc \
  --env POSTGRES_PASSWORD=ohc \
  --env POSTGRES_DB=ohc \
  --port 5432
kubectl expose pod postgres --namespace "${NAMESPACE}" --port 5432
kubectl wait pod/postgres \
  --namespace "${NAMESPACE}" \
  --for=condition=Ready \
  --timeout=120s

# ── Install Redis for cloud/web mesh and cache paths ───────────────────────────
log "Installing Redis ..."
helm upgrade --install redis bitnami/redis \
  --namespace "${NAMESPACE}" \
  --set architecture=standalone \
  --set auth.enabled=false \
  --wait --timeout 120s

# ── Cloud/web mode ─────────────────────────────────────────────────────────────
install_ohc_release "${CLOUD_RELEASE_NAME}" "cloud/web mode" "${CLOUD_HELM_SMOKE_ARGS[@]}"
run_rest_smoke_tests "${CLOUD_RELEASE_NAME}" "cloud/web mode" 18080

log "Removing cloud/web release before standalone smoke ..."
helm uninstall "${CLOUD_RELEASE_NAME}" --namespace "${NAMESPACE}"

# ── Standalone/desktop mode ────────────────────────────────────────────────────
install_ohc_release "${STANDALONE_RELEASE_NAME}" "standalone/desktop mode" "${STANDALONE_HELM_SMOKE_ARGS[@]}"
run_rest_smoke_tests "${STANDALONE_RELEASE_NAME}" "standalone/desktop mode" 18081

log ""
log "All Kind e2e smoke tests passed in cloud/web and standalone/desktop modes."

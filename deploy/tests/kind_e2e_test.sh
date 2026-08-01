#!/usr/bin/env bash
# Kind cluster end-to-end smoke test for the OHC platform.
#
# This test:
#   1. Creates a temporary Kind cluster
#   2. Builds and loads Docker images into the cluster
#   3. Installs Valkey and PostgreSQL for cloud/web mode
#   4. Installs the OHC application chart in cloud/web mode
#   5. Runs REST API smoke tests
#   6. Installs the OHC application chart in standalone/desktop mode
#   7. Runs the same REST API smoke tests against SQLite-backed standalone mode
#   8. Cleans up the cluster on exit
#
# Prerequisites (on PATH):
#   kind, helm, kubectl, docker, curl, openssl
set -euo pipefail
umask 077

CLUSTER_NAME="ohc-e2e-$$"
NAMESPACE="ohc-e2e"
CLOUD_RELEASE_NAME="ohc-cloud"
STANDALONE_RELEASE_NAME="ohc-standalone"
GRPC_TLS_SECRET_NAME="ohc-e2e-grpc-tls"
GRPC_TLS_DIR="${TEST_TMPDIR:-/tmp}/ohc-grpc-tls-$$"
GRPC_PROBE=""
SETUP_SECRET_NAME="ohc-e2e-setup"
AUTH_SECRET_NAME="ohc-e2e-auth"
AGENT_AUTH_SECRET_NAME="ohc-e2e-agent-auth"
ADMIN_USERNAME="kind-e2e-admin"
ADMIN_EMAIL="kind-e2e-admin@example.test"
ADMIN_ORGANIZATION_ID="kind-e2e-org"
FAILED_COMMAND=""
FAILED_LINE=""

log() { echo "[kind-e2e] $*"; }

curl_bounded() {
  command curl \
    --connect-timeout 5 \
    --max-time 30 \
    --retry 5 \
    --retry-delay 1 \
    --retry-connrefused \
    "$@"
}

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

ensure_image_loaded_in_kind() {
  local image="$1"
  log "Ensuring image ${image} is loaded into Kind cluster ..."
  # Always attempt to pull inside the Kind node directly first to avoid Docker multi-platform archive issues
  log "Pulling image directly inside the Kind node using crictl ..."
  if ! docker exec "${CLUSTER_NAME}-control-plane" crictl pull "${image}" 2>/dev/null; then
    log "crictl pull failed; falling back to host pull and kind load ..."
    if ! docker image inspect "${image}" >/dev/null 2>&1; then
      log "Image ${image} not found locally. Pulling ..."
      docker pull "${image}"
    fi
    kind load docker-image "${image}" --name "${CLUSTER_NAME}"
  fi
}

cleanup() {
  if [[ -n "${PF_PID:-}" ]]; then
    kill "${PF_PID}" 2>/dev/null || true
  fi
  log "Deleting Kind cluster ${CLUSTER_NAME} ..."
  kind delete cluster --name "${CLUSTER_NAME}" 2>/dev/null || true
  rm -rf "${GRPC_TLS_DIR}" "${CHART_DIR:-}"
  if [[ -n "${KUBECONFIG:-}" ]]; then
    rm -f "${KUBECONFIG}"
  fi
}

dump_diagnostics() {
  if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    echo "::group::Kind E2E failure diagnostics"
  fi

  log "Collecting Kubernetes diagnostics after failure ..."
  if [[ -n "${FAILED_COMMAND}" ]]; then
    echo "Failed command near line ${FAILED_LINE}: ${FAILED_COMMAND}" >&2
  fi

  helm list --all-namespaces 2>/dev/null || true
  kubectl get nodes -o wide 2>/dev/null || true
  kubectl get pods --namespace "${NAMESPACE}" -o wide 2>/dev/null || true
  kubectl get deployments --namespace "${NAMESPACE}" -o wide 2>/dev/null || true
  kubectl get services --namespace "${NAMESPACE}" -o wide 2>/dev/null || true
  kubectl get pvc --namespace "${NAMESPACE}" -o wide 2>/dev/null || true
  kubectl get events --namespace "${NAMESPACE}" --sort-by='.lastTimestamp' 2>/dev/null || true
  kubectl describe pods --namespace "${NAMESPACE}" 2>/dev/null || true
  kubectl logs --namespace "${NAMESPACE}" --all-containers --tail=100 -l app.kubernetes.io/name=valkey 2>/dev/null || true
  kubectl logs --namespace "${NAMESPACE}" --all-containers --tail=100 -l "app=${CLOUD_RELEASE_NAME}-backend" 2>/dev/null || true
  kubectl logs --namespace "${NAMESPACE}" --all-containers --previous --tail=200 -l "app=${CLOUD_RELEASE_NAME}-backend" 2>/dev/null || true
  kubectl logs --namespace "${NAMESPACE}" --all-containers --tail=100 -l "app=${STANDALONE_RELEASE_NAME}-backend" 2>/dev/null || true

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

# ── Prerequisites ──────────────────────────────────────────────────────────────
for tool in kind helm jq kubectl docker curl openssl timeout; do
  require_tool "${tool}"
done
SETUP_TOKEN="$(openssl rand -hex 32)"
JWT_SECRET="$(openssl rand -hex 32)"
AGENT_TOKEN="$(openssl rand -hex 32)"
AGENT_AUTH_KEY="$(openssl rand -hex 32)"
ADMIN_PASSWORD="OHC-E2E-Aa1-$(openssl rand -hex 24)"

# ── Locate repo root (works both inside and outside Bazel sandbox) ────────────
if [[ -n "${TEST_SRCDIR:-}" ]]; then
  workspace="${TEST_WORKSPACE:-mono}"
  REPO_ROOT="${TEST_SRCDIR}/${workspace}"
else
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
fi

log "Repo root: ${REPO_ROOT}"

CHART_DIR="$(mktemp -d "${TEST_TMPDIR:-/tmp}/ohc-chart.XXXXXX")"
cp -RL "${REPO_ROOT}/deploy/helm/ohc/." "${CHART_DIR}/"
chmod -R u+w "${CHART_DIR}"

COMMON_HELM_SMOKE_ARGS=(
  --set backend.replicas=1
  --set backend.autoscaling.enabled=false
  --set backend.vpa.enabled=false
  --set backend.resources.requests.cpu=500m
  --set backend.resources.requests.memory=256Mi
  --set backend.resources.limits.cpu=2
  --set backend.resources.limits.memory=1Gi
  --set valkey.enabled=false
  --set cnpg.enabled=false
  --set ohcCore.enabled=false
  --set powersync.enabled=false
  --set kube-prometheus-stack.enabled=false
  --set fluentBit.enabled=false
  --set resourceQuota.enabled=false
  --set-string backend.setup.existingSecret=${SETUP_SECRET_NAME}
  --set-string backend.auth.existingSecret=${AUTH_SECRET_NAME}
)

CLOUD_HELM_SMOKE_ARGS=(
  "${COMMON_HELM_SMOKE_ARGS[@]}"
  --set multiTenant.enabled=true
  --set valkey.enabled=true
  --set valkey.image.tag=8-alpine
  --set-string backend.grpcTls.existingSecret=${GRPC_TLS_SECRET_NAME}
  --set-string backend.env.DATABASE_URL=postgres://ohc:ohc@postgres:5432/ohc
  --set-string backend.env.OHC_STANDALONE_MODE=false
  --set-string backend.env.OHC_AUTH_RATE_LIMIT_DEPLOYMENT=single-instance
)

STANDALONE_HELM_SMOKE_ARGS=(
  "${COMMON_HELM_SMOKE_ARGS[@]}"
  --set multiTenant.enabled=false
  --set-string backend.env.DATABASE_URL=sqlite:///tmp/ohc-standalone/standalone.db
  --set-string backend.env.OHC_SQLITE_KEY=kind-e2e-standalone-sqlite-key
  --set-string backend.env.OHC_STANDALONE_MODE=true
  --set-string backend.env.OHC_TELEMETRY_ENABLED=false
  --set-string backend.agentAuth.existingSecret=${AGENT_AUTH_SECRET_NAME}
)

# ── Create Kind cluster ────────────────────────────────────────────────────────
log "Creating Kind cluster '${CLUSTER_NAME}' ..."

# Set KUBECONFIG to a temporary file BEFORE creating the cluster to avoid
# trying to lock the user's read-only default kubeconfig in the sandbox.
export KUBECONFIG="$(mktemp "${TEST_TMPDIR:-/tmp}/kind-kubeconfig.XXXXXX")"
chmod 600 "${KUBECONFIG}"

kind create cluster --name "${CLUSTER_NAME}" --image kindest/node:v1.29.2 --wait 120s

log "Waiting for cluster nodes ..."
kubectl wait --for=condition=Ready node --all --timeout=120s

# ── Locating Images ────────────────────────────────────────────────────────────
# If running under Bazel, we use the pre-built image loaders.
# In a manual run, we fallback to docker build (for dev convenience).
if [[ -n "${TEST_SRCDIR:-}" ]]; then
  log "Bazel environment detected. Loading images from runfiles..."
  SERVER_LOADER="${REPO_ROOT}/deploy/load_all_images"
  GRPC_PROBE="${REPO_ROOT}/deploy/grpc_mtls_probe"

  if [[ ! -f "${SERVER_LOADER}" || ! -x "${SERVER_LOADER}" ]]; then
    SERVER_LOADER="$(find "${TEST_SRCDIR}" -name "load_all_images" -type f -executable | head -1)"
  fi

  if [[ -z "${SERVER_LOADER}" || ! -x "${SERVER_LOADER}" ]]; then
    echo "error: could not find executable load_all_images in Bazel runfiles" >&2
    exit 1
  fi
  if [[ ! -x "${GRPC_PROBE}" ]]; then
    GRPC_PROBE="$(find "${TEST_SRCDIR}" -name grpc_mtls_probe -type f -executable | head -1)"
  fi
  if [[ -z "${GRPC_PROBE}" || ! -x "${GRPC_PROBE}" ]]; then
    echo "error: could not find executable grpc_mtls_probe in Bazel runfiles" >&2
    exit 1
  fi

  log "Executing server loader: ${SERVER_LOADER}"
  "${SERVER_LOADER}"
  docker tag onehumancorp/server:latest onehumancorp/server:e2e
else
  require_tool bazelisk
  log "Manual run detected. Building server image via Bazel..."
  bazelisk run //deploy:server_load
  bazelisk build //deploy:grpc_mtls_probe
  GRPC_PROBE="$(bazelisk cquery --output=files //deploy:grpc_mtls_probe | head -1)"
  docker tag onehumancorp/server:latest onehumancorp/server:e2e
fi

# ── Add Helm repos ─────────────────────────────────────────────────────────────
log "Adding Helm repos ..."
helm repo add valkey https://valkey.io/valkey-helm/ 2>/dev/null || true
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts 2>/dev/null || true
helm repo update valkey prometheus-community 2>/dev/null || true

log "Building chart dependencies ..."
helm dependency build "${CHART_DIR}" --skip-refresh

wait_for_backend() {
  local backend_url="$1"
  local max_attempts=30
  local attempt=0
  while (( attempt < max_attempts )); do
    if curl_bounded -sf "${backend_url}/healthz" >/dev/null 2>&1; then
      return 0
    fi
    attempt=$((attempt + 1))
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

verify_grpc_mtls() {
  local grpc_port="$1"
  local authenticated_tls
  if ! authenticated_tls="$(timeout 10 openssl s_client \
    -connect "127.0.0.1:${grpc_port}" \
    -servername localhost \
    -verify_return_error \
    -verify_hostname localhost \
    -alpn h2 \
    -CAfile "${GRPC_TLS_DIR}/ca.crt" \
    -cert "${GRPC_TLS_DIR}/client.crt" \
    -key "${GRPC_TLS_DIR}/client.key" </dev/null 2>&1)"; then
    echo "error: gRPC TLS listener rejected or timed out for a CA-signed client certificate" >&2
    return 1
  fi
  if ! grep -Fq 'Verify return code: 0 (ok)' <<<"${authenticated_tls}" || \
     ! grep -Fq 'ALPN protocol: h2' <<<"${authenticated_tls}"; then
    echo "error: gRPC listener did not negotiate a verified HTTP/2 TLS session" >&2
    return 1
  fi
  "${GRPC_PROBE}" "https://localhost:${grpc_port}" \
    "${GRPC_TLS_DIR}/ca.crt" - - tls-rejected
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
    --timeout=180s

  kubectl wait pod \
    --namespace "${NAMESPACE}" \
    -l "app=${release_name}-backend" \
    --for=condition=Ready \
    --timeout=180s
}

run_rest_smoke_tests() {
  local release_name="$1"
  local mode_name="$2"
  local local_port="$3"
  local grpc_local_port=$((local_port + 1000))
  local backend_url="http://127.0.0.1:${local_port}"
  local port_mappings=("${local_port}:8080")
  if [[ "${mode_name}" == "cloud/web mode" ]]; then
    port_mappings+=("${grpc_local_port}:8081")
  fi

  log "Port-forwarding ${mode_name} backend service in a loop ..."
  stop_port_forward
  (
    while true; do
      kubectl port-forward \
        --namespace "${NAMESPACE}" \
        "svc/${release_name}-backend" \
        "${port_mappings[@]}" >/dev/null 2>&1 || true
      sleep 1
    done
  ) &
  PF_PID=$!

  sleep 3

  log "Waiting for ${mode_name} backend /healthz ..."
  wait_for_backend "${backend_url}"

  if [[ "${mode_name}" == "cloud/web mode" ]]; then
    log "Verifying ${mode_name} gRPC mutual TLS handshake ..."
    verify_grpc_mtls "${grpc_local_port}"
  fi

  log "Running ${mode_name} REST smoke tests ..."

# --- health check ---
  response="$(curl_bounded -sf "${backend_url}/healthz")"
  [[ "${response}" == "ok" ]] || { echo "healthz failed: ${response}" >&2; exit 1; }
  log "  /healthz ✓"

  response="$(curl_bounded -sf "${backend_url}/readyz")"
  [[ "${response}" == "ok" ]] || { echo "readyz failed: ${response}" >&2; exit 1; }
  log "  /readyz ✓"

# --- one-time setup and normal login ---
  wrong_setup_status="$(curl_bounded -sS -o /dev/null -w '%{http_code}' \
    -X POST "${backend_url}/api/v1/setup/admin" \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer wrong-setup-token-at-least-32-bytes' \
    -d "{\"username\":\"${ADMIN_USERNAME}\",\"email\":\"${ADMIN_EMAIL}\",\"password\":\"${ADMIN_PASSWORD}\",\"organizationId\":\"${ADMIN_ORGANIZATION_ID}\"}")"
  [[ "${wrong_setup_status}" == "401" ]] || {
    echo "wrong-setup-token was not denied: HTTP ${wrong_setup_status}" >&2
    exit 1
  }
  missing_setup_status="$(curl_bounded -sS -o /dev/null -w '%{http_code}' \
    -X POST "${backend_url}/api/v1/setup/admin" \
    -H 'Content-Type: application/json' \
    -d "{\"username\":\"${ADMIN_USERNAME}\",\"email\":\"${ADMIN_EMAIL}\",\"password\":\"${ADMIN_PASSWORD}\",\"organizationId\":\"${ADMIN_ORGANIZATION_ID}\"}")"
  [[ "${missing_setup_status}" == "401" ]] || {
    echo "setup without a token was not denied: HTTP ${missing_setup_status}" >&2
    exit 1
  }

  curl_bounded -sf -X POST "${backend_url}/api/v1/setup/admin" \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer ${SETUP_TOKEN}" \
    -d "{\"username\":\"${ADMIN_USERNAME}\",\"email\":\"${ADMIN_EMAIL}\",\"password\":\"${ADMIN_PASSWORD}\",\"organizationId\":\"${ADMIN_ORGANIZATION_ID}\"}" >/dev/null
  log "  initial admin setup ✓"

  login_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/auth/login" \
    -H 'Content-Type: application/json' \
    -d "{\"username\":\"${ADMIN_USERNAME}\",\"password\":\"${ADMIN_PASSWORD}\",\"organization_id\":\"${ADMIN_ORGANIZATION_ID}\"}")"
  if ! access_token="$(printf '%s' "${login_response}" | jq -er '.token | select(type == "string" and length > 0)')"; then
    echo "error: login response did not contain a nonempty JWT" >&2
    exit 1
  fi
  auth_headers=(-H "Authorization: Bearer ${access_token}")
  log "  authenticated login ✓"

  if [[ "${mode_name}" == "cloud/web mode" ]]; then
    log "Verifying a real SPIFFE-intercepted gRPC request ..."
    "${GRPC_PROBE}" "https://localhost:${grpc_local_port}" \
      "${GRPC_TLS_DIR}/ca.crt" \
      "${GRPC_TLS_DIR}/client.crt" \
      "${GRPC_TLS_DIR}/client.key" \
      success "${ADMIN_ORGANIZATION_ID}"
    "${GRPC_PROBE}" "https://localhost:${grpc_local_port}" \
      "${GRPC_TLS_DIR}/ca.crt" \
      "${GRPC_TLS_DIR}/client-no-spiffe.crt" \
      "${GRPC_TLS_DIR}/client-no-spiffe.key" \
      unauthenticated "${ADMIN_ORGANIZATION_ID}"
  fi

  protected_status="$(curl_bounded -sS -o /dev/null -w '%{http_code}' \
    -H 'Authorization: Bearer wrong-jwt' "${backend_url}/api/v1/dashboard")"
  [[ "${protected_status}" == "401" ]] || {
    echo "wrong-jwt was not denied: HTTP ${protected_status}" >&2
    exit 1
  }
  missing_jwt_status="$(curl_bounded -sS -o /dev/null -w '%{http_code}' \
    "${backend_url}/api/v1/dashboard")"
  [[ "${missing_jwt_status}" == "401" ]] || {
    echo "protected API without a JWT was not denied: HTTP ${missing_jwt_status}" >&2
    exit 1
  }

# --- seed demo data ---
  seed_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/dev/seed" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d '{"scenario":"launch-readiness"}')"
  printf '%s' "${seed_response}" | jq -e '.ok == true' >/dev/null
  log "  /api/v1/dev/seed ✓"

# --- dashboard ---
  dashboard="$(curl_bounded -sf "${auth_headers[@]}" "${backend_url}/api/v1/dashboard")"
  echo "${dashboard}" | grep -q '"organization"' || { echo "dashboard missing 'organization'" >&2; exit 1; }
  log "  /api/v1/dashboard ✓"

# --- agents list ---
  agents="$(curl_bounded -sf "${auth_headers[@]}" "${backend_url}/api/v1/agents")"
  echo "${agents}" | grep -q '\[' || { echo "agents response not a JSON array" >&2; exit 1; }
  log "  /api/v1/agents ✓"

# --- hire agent ---
  hire_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/agents/hire" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d '{"name":"E2E Test Agent","role":"SOFTWARE_ENGINEER","model":"gpt-4o-mini"}')"
  echo "${hire_response}" | grep -q '"id"' || { echo "hire agent failed: ${hire_response}" >&2; exit 1; }
  log "  /api/v1/agents/hire ✓"

# --- meetings ---
  meetings="$(curl_bounded -sf "${auth_headers[@]}" "${backend_url}/api/v1/meetings")"
  echo "${meetings}" | grep -q '\[' || { echo "meetings response not a JSON array" >&2; exit 1; }
  log "  /api/v1/meetings ✓"

# --- costs ---
  costs="$(curl_bounded -sf "${auth_headers[@]}" "${backend_url}/api/v1/costs")"
  echo "${costs}" | grep -q '"totalCostUSD"' || { echo "costs missing totalCostUSD" >&2; exit 1; }
  log "  /api/v1/costs ✓"

# --- approval flow ---
  approval_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/approvals/request" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d '{"agentId":"swe-1","action":"deploy-to-production","reason":"E2E test","estimatedCostUsd":0.01,"riskLevel":"low"}')"
  echo "${approval_response}" | grep -q '"id"' || { echo "approval create failed: ${approval_response}" >&2; exit 1; }
  approval_id="$(echo "${approval_response}" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)"
  log "  /api/v1/approvals/request ✓ (id=${approval_id})"

  curl_bounded -sf -X PUT "${backend_url}/api/v1/approvals/decide" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d "{\"approvalId\":\"${approval_id}\",\"decision\":\"approve\",\"decidedBy\":\"e2e-test\"}" >/dev/null
  log "  /api/v1/approvals/decide ✓"

# --- warm handoff ---
  handoff_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/handoffs" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d '{"fromAgentId":"swe-1","toHumanRole":"MANAGER","intent":"need-review","failedAttempts":1,"currentState":"blocked"}')"
  echo "${handoff_response}" | grep -q '"id"' || { echo "handoff create failed: ${handoff_response}" >&2; exit 1; }
  log "  /api/v1/handoffs ✓"

# --- billing costs ---
  costs2="$(curl_bounded -sf "${auth_headers[@]}" "${backend_url}/api/v1/costs")"
  echo "${costs2}" | grep -q '"totalCostUSD"' || { echo "costs2 missing totalCostUSD" >&2; exit 1; }
  log "  /api/v1/costs (post-hire) ✓"

# --- skill pack import ---
  skill_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/skills/import" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d '{"name":"E2E Skill Pack","domain":"testing","description":"e2e","source":"custom","roles":[{"role":"SOFTWARE_ENGINEER","basePrompt":"e2e prompt"}]}')"
  echo "${skill_response}" | grep -q '"id"' || { echo "skill import failed: ${skill_response}" >&2; exit 1; }
  log "  /api/v1/skills/import ✓"

# --- org snapshot ---
  snapshot_response="$(curl_bounded -sf -X POST "${backend_url}/api/v1/snapshots/create" \
    "${auth_headers[@]}" \
    -H 'Content-Type: application/json' \
    -d '{"label":"e2e-snapshot"}')"
  echo "${snapshot_response}" | grep -q '"id"' || { echo "snapshot create failed: ${snapshot_response}" >&2; exit 1; }
  log "  /api/v1/snapshots/create ✓"

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
  ensure_image_loaded_in_kind postgres:15-alpine
  ensure_image_loaded_in_kind valkey/valkey:8-alpine@sha256:94365b275456ae14621001c03556c732b1d93a0cdeacc317d1bdd52eba680885

# ── Create namespace ───────────────────────────────────────────────────────────
kubectl create namespace "${NAMESPACE}" --dry-run=client -o yaml | kubectl apply -f -

log "Creating ephemeral setup Secret ..."
kubectl create secret generic "${SETUP_SECRET_NAME}" \
  --namespace "${NAMESPACE}" \
  --from-literal=token="${SETUP_TOKEN}" \
  --dry-run=client -o yaml | kubectl apply -f -

log "Creating ephemeral authentication Secret ..."
kubectl create secret generic "${AUTH_SECRET_NAME}" \
  --namespace "${NAMESPACE}" \
  --from-literal=jwtSecret="${JWT_SECRET}" \
  --dry-run=client -o yaml | kubectl apply -f -

log "Creating ephemeral built-in agent authentication Secret ..."
kubectl create secret generic "${AGENT_AUTH_SECRET_NAME}" \
  --namespace "${NAMESPACE}" \
  --from-literal=agentToken="${AGENT_TOKEN}" \
  --from-literal=authKey="${AGENT_AUTH_KEY}" \
  --dry-run=client -o yaml | kubectl apply -f -

# Cloud mode requires mTLS for its gRPC listener. Generate a short-lived test
# CA and server identity, then supply them through the same existing-Secret
# interface used by real deployments.
log "Creating ephemeral gRPC TLS Secret ..."
umask 077
mkdir -p "${GRPC_TLS_DIR}"
openssl req -x509 -newkey rsa:2048 -nodes -sha256 -days 1 \
  -keyout "${GRPC_TLS_DIR}/ca.key" \
  -out "${GRPC_TLS_DIR}/ca.crt" \
  -subj "/CN=ohc-kind-e2e-ca" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,keyCertSign,cRLSign" >/dev/null 2>&1
openssl req -new -newkey rsa:2048 -nodes -sha256 \
  -keyout "${GRPC_TLS_DIR}/tls.key" \
  -out "${GRPC_TLS_DIR}/server.csr" \
  -subj "/CN=${CLOUD_RELEASE_NAME}-backend" >/dev/null 2>&1
printf '%s\n' \
  "subjectAltName=DNS:${CLOUD_RELEASE_NAME}-backend,DNS:${CLOUD_RELEASE_NAME}-backend.${NAMESPACE}.svc,DNS:localhost,IP:127.0.0.1" \
  "extendedKeyUsage=serverAuth" > "${GRPC_TLS_DIR}/server.ext"
openssl x509 -req -sha256 -days 1 \
  -in "${GRPC_TLS_DIR}/server.csr" \
  -CA "${GRPC_TLS_DIR}/ca.crt" \
  -CAkey "${GRPC_TLS_DIR}/ca.key" \
  -CAcreateserial \
  -out "${GRPC_TLS_DIR}/tls.crt" \
  -extfile "${GRPC_TLS_DIR}/server.ext" >/dev/null 2>&1
openssl req -new -newkey rsa:2048 -nodes -sha256 \
  -keyout "${GRPC_TLS_DIR}/client.key" \
  -out "${GRPC_TLS_DIR}/client.csr" \
  -subj '/CN=kind-e2e-client' >/dev/null 2>&1
printf '%s\n' \
  'basicConstraints=critical,CA:FALSE' \
  'keyUsage=critical,digitalSignature,keyEncipherment' \
  'extendedKeyUsage=clientAuth' \
  "subjectAltName=URI:spiffe://ohc.local/org/${ADMIN_ORGANIZATION_ID}/agent/e2e-client" \
  > "${GRPC_TLS_DIR}/client.ext"
openssl x509 -req -sha256 -days 1 \
  -in "${GRPC_TLS_DIR}/client.csr" \
  -CA "${GRPC_TLS_DIR}/ca.crt" \
  -CAkey "${GRPC_TLS_DIR}/ca.key" \
  -CAcreateserial \
  -out "${GRPC_TLS_DIR}/client.crt" \
  -extfile "${GRPC_TLS_DIR}/client.ext" >/dev/null 2>&1
openssl req -new -newkey rsa:2048 -nodes -sha256 \
  -keyout "${GRPC_TLS_DIR}/client-no-spiffe.key" \
  -out "${GRPC_TLS_DIR}/client-no-spiffe.csr" \
  -subj '/CN=kind-e2e-client-without-spiffe' >/dev/null 2>&1
printf '%s\n' \
  'basicConstraints=critical,CA:FALSE' \
  'keyUsage=critical,digitalSignature,keyEncipherment' \
  'extendedKeyUsage=clientAuth' \
  > "${GRPC_TLS_DIR}/client-no-spiffe.ext"
openssl x509 -req -sha256 -days 1 \
  -in "${GRPC_TLS_DIR}/client-no-spiffe.csr" \
  -CA "${GRPC_TLS_DIR}/ca.crt" \
  -CAkey "${GRPC_TLS_DIR}/ca.key" \
  -CAcreateserial \
  -out "${GRPC_TLS_DIR}/client-no-spiffe.crt" \
  -extfile "${GRPC_TLS_DIR}/client-no-spiffe.ext" >/dev/null 2>&1
kubectl create secret generic "${GRPC_TLS_SECRET_NAME}" \
  --namespace "${NAMESPACE}" \
  --from-file=tls.crt="${GRPC_TLS_DIR}/tls.crt" \
  --from-file=tls.key="${GRPC_TLS_DIR}/tls.key" \
  --from-file=ca.crt="${GRPC_TLS_DIR}/ca.crt" \
  --dry-run=client -o yaml | kubectl apply -f -

# ── Install PostgreSQL for the cloud/web backend smoke test ───────────────────
log "Installing PostgreSQL ..."
kubectl run postgres \
  --namespace "${NAMESPACE}" \
  --image postgres:15-alpine \
  --env POSTGRES_USER=ohc \
  --env POSTGRES_PASSWORD=ohc \
  --env POSTGRES_DB=ohc \
  --port 5432
kubectl expose pod postgres --namespace "${NAMESPACE}" --port 5432
kubectl wait pod/postgres \
  --namespace "${NAMESPACE}" \
  --for=condition=Ready \
  --timeout=120s

log "Waiting for PostgreSQL server to be ready for connections ..."
pg_attempts=0
pg_max_attempts=30
while (( pg_attempts < pg_max_attempts )); do
  if kubectl exec --namespace "${NAMESPACE}" postgres -- pg_isready -U ohc -d ohc >/dev/null 2>&1; then
    log "PostgreSQL is ready!"
    break
  fi
  pg_attempts=$((pg_attempts + 1))
  sleep 2
done
if (( pg_attempts == pg_max_attempts )); then
  echo "error: PostgreSQL did not become ready for connections" >&2
  exit 1
fi

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

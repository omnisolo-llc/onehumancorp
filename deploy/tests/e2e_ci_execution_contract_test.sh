#!/usr/bin/env bash
set -euo pipefail

kind_script="$1"
compose_script="$2"
backend_template="$3"
chart_values="$4"
bootstrap_script="$5"
compose_manifest="$6"

require_literal() {
  local needle="$1"
  local file="$2"
  local message="$3"
  grep -Fq -- "${needle}" "${file}" || {
    echo "${message}" >&2
    exit 1
  }
}

require_literal 'startupProbe:' "$backend_template" \
  "Backend deployment must protect first-run migrations with a startup probe"
require_literal 'backend.agentAuth.existingSecret is required in standalone mode' "$backend_template" \
  "Standalone Helm deployments must require Secret-backed built-in-agent authentication."
require_literal 'key: agentToken' "$backend_template" \
  "Helm backend deployment must read its built-in-agent token from the fixed Secret key."
require_literal '- name: OHC_AGENT_AUTH_KEY' "$backend_template" \
  "Helm backend deployment must project the standalone built-in-agent HMAC key."
require_literal 'key: authKey' "$backend_template" \
  "Helm backend deployment must read its built-in-agent HMAC key from the fixed Secret key."
require_literal '--from-literal=authKey="${AGENT_AUTH_KEY}"' "$kind_script" \
  "Kind E2E must populate the standalone built-in-agent HMAC key."
require_literal 'backend.env.OHC_AGENT_AUTH_KEY' "$backend_template" \
  "Helm must reject inline standalone built-in-agent HMAC keys."

reject_literal() {
  local needle="$1"
  local file="$2"
  local message="$3"
  if grep -Fq -- "${needle}" "${file}"; then
    echo "${message}" >&2
    exit 1
  fi
}

for script in "$kind_script" "$compose_script"; do
  if grep -Fq 'Skipping test in CI environment' "$script"; then
    echo "${script} still bypasses its real smoke test in CI." >&2
    exit 1
  fi
  if grep -F '${backend_url}/api/' "$script" | grep -Fvq '${backend_url}/api/v1/'; then
    echo "${script} still calls an unversioned API endpoint." >&2
    exit 1
  fi
  if grep -F '${BASE_URL}/api/' "$script" | grep -Fvq '${BASE_URL}/api/v1/'; then
    echo "${script} still calls an unversioned API endpoint." >&2
    exit 1
  fi
done

grep -Fq 'ensure_image_loaded_in_kind pgvector/pgvector:pg16' "$kind_script" || {
  echo "Kind E2E must load the same canonical pgvector image that CI pre-pulls." >&2
  exit 1
}
grep -Fq 'ensure_image_loaded_in_kind valkey/valkey:8-alpine@sha256:94365b275456ae14621001c03556c732b1d93a0cdeacc317d1bdd52eba680885' "$kind_script" || {
  echo "Kind E2E must load the same canonical Valkey image that CI pre-pulls." >&2
  exit 1
}
grep -Fq 'crictl pull "${image}"' "$kind_script" || {
  echo "Kind E2E must fall back to a native-platform pull inside the Kind node." >&2
  exit 1
}

grep -Fq 'cp -RL "${REPO_ROOT}/deploy/helm/ohc/." "${CHART_DIR}/"' "$kind_script" || {
  echo "Kind E2E must copy Helm chart contents into its writable temporary chart root" >&2
  exit 1
}

grep -Fq 'kube-prometheus-stack' "$(dirname "$backend_template")/prometheusrule.yaml" || {
  echo "PrometheusRule must be gated by the kube-prometheus-stack feature flag" >&2
  exit 1
}

if grep -Fq 'podSelector: {}' "$(dirname "$backend_template")/default-deny-network-policy.yaml"; then
  echo "Helm default-deny policy must not select unrelated namespace workloads" >&2
  exit 1
fi

if grep -Fq 'mirror.gcr.io/pgvector/pgvector:pg15' "$kind_script"; then
  echo "Kind E2E still references the obsolete pgvector mirror tag." >&2
  exit 1
fi

grep -Fq 'existingSecret:' "$chart_values" || {
  echo "Helm values must expose an existing Secret for backend gRPC TLS." >&2
  exit 1
}
for variable in OHC_GRPC_TLS_CERT_PATH OHC_GRPC_TLS_KEY_PATH OHC_GRPC_CLIENT_CA_PATH; do
  grep -Fq -- "- name: ${variable}" "$backend_template" || {
    echo "Helm backend deployment does not configure ${variable}." >&2
    exit 1
  }
done
require_literal 'name: grpc' "$backend_template" \
  "Helm backend deployment must expose its gRPC listener."
grep -Fq 'readOnly: true' "$backend_template" || {
  echo "Backend gRPC TLS Secret must be mounted read-only." >&2
  exit 1
}
grep -Fq 'secretName:' "$backend_template" || {
  echo "Backend gRPC TLS volume must reference the configured Secret." >&2
  exit 1
}
for key in tls.crt tls.key ca.crt; do
  grep -Fq "key: ${key}" "$backend_template" || {
    echo "Backend gRPC TLS volume must project only its required ${key} key." >&2
    exit 1
  }
done

grep -Fq 'kubectl create secret generic "${GRPC_TLS_SECRET_NAME}"' "$kind_script" || {
  echo "Kind E2E must create an ephemeral gRPC TLS Secret." >&2
  exit 1
}
grep -Fq 'backend.grpcTls.existingSecret=${GRPC_TLS_SECRET_NAME}' "$kind_script" || {
  echo "Kind cloud smoke must install the chart with its gRPC TLS Secret." >&2
  exit 1
}
grep -Fq 'backend.env.OHC_AUTH_RATE_LIMIT_DEPLOYMENT=single-instance' "$kind_script" || {
  echo "Kind cloud smoke must declare its single-instance auth rate-limit topology." >&2
  exit 1
}
grep -Fq 'backend.agentAuth.existingSecret=${AGENT_AUTH_SECRET_NAME}' "$kind_script" || {
  echo "Kind standalone smoke must install the chart with its built-in-agent authentication Secret." >&2
  exit 1
}

# The one-time bootstrap helper must fail closed and use only the versioned,
# setup-token-protected endpoint.
require_literal 'read_secret OHC_SETUP_TOKEN OHC_SETUP_TOKEN_FILE' "$bootstrap_script" \
  "Bootstrap helper must continue to support direct and file-backed setup tokens."
require_literal 'OHC_SETUP_TOKEN_FILE' "$bootstrap_script" \
  "Bootstrap helper must support a setup-token secret file."
require_literal 'SETUP_ADMIN_INIT_PASSWORD_FILE' "$bootstrap_script" \
  "Bootstrap helper must support an admin-password secret file."
require_literal '32' "$bootstrap_script" \
  "Bootstrap helper must reject setup tokens below the server minimum."
for variable in SETUP_ADMIN_INIT_USERNAME SETUP_ADMIN_INIT_EMAIL SETUP_ADMIN_INIT_ORGANIZATION_ID; do
  require_literal "\${${variable}:?" "$bootstrap_script" \
    "Bootstrap helper must require ${variable}."
done
require_literal 'read_secret SETUP_ADMIN_INIT_PASSWORD SETUP_ADMIN_INIT_PASSWORD_FILE' "$bootstrap_script" \
  "Bootstrap helper must require a direct or file-backed admin password."
require_literal '${SERVER_URL}/api/v1/setup/admin' "$bootstrap_script" \
  "Bootstrap helper must call the versioned setup endpoint."
require_literal '${SERVER_URL}/api/v1/auth/login' "$bootstrap_script" \
  "Bootstrap helper must verify configured admin credentials after an already-initialized response."
require_literal 'Authorization: Bearer ${OHC_SETUP_TOKEN}' "$bootstrap_script" \
  "Bootstrap helper must authenticate with the setup bearer token."
require_literal '--connect-timeout' "$bootstrap_script" \
  "Bootstrap helper network calls must have a connection deadline."
require_literal '--max-time' "$bootstrap_script" \
  "Bootstrap helper network calls must have an overall deadline."
require_literal '--data-binary' "$bootstrap_script" \
  "Bootstrap helper must submit its JSON from a private request file."
reject_literal '--post-data' "$bootstrap_script" \
  "Bootstrap helper must not expose password-bearing JSON in argv."
if grep -Eq -- '--post-data .*"role"' "$bootstrap_script"; then
  echo "Bootstrap request must not control the initial user's role." >&2
  exit 1
fi

# Exercise the unexpected-response path with a fake wget. A 500 must return
# nonzero, leave no marker, and avoid echoing any supplied secret.
contract_tmp="$(mktemp -d)"
trap 'rm -rf "${contract_tmp}"' EXIT
mkdir -p "${contract_tmp}/bin" "${contract_tmp}/secrets"
cp "$bootstrap_script" "${contract_tmp}/bootstrap-admin.sh"
cat > "${contract_tmp}/bin/curl" <<'EOF'
#!/bin/sh
for argument in "$@"; do
  case "$argument" in
  */readyz)
    exit 0
    ;;
  esac
done
printf '%s\n' "$@" > "${BOOTSTRAP_CONTRACT_REQUEST:?}"
output=""
request_url=""
while [ "$#" -gt 0 ]; do
  case "$1" in
  */api/v1/*)
    request_url="$1"
    ;;
  esac
  if [ "$1" = "--output" ] || [ "$1" = "-o" ]; then
    shift
    output="$1"
  fi
  shift
done
[ -z "$output" ] || printf '%s\n' '{"error":"forced contract failure"}' > "$output"
case "$request_url" in
*/api/v1/auth/login)
  printf '%s' "${BOOTSTRAP_CONTRACT_LOGIN_STATUS:-401}"
  ;;
*)
  printf '%s' "${BOOTSTRAP_CONTRACT_SETUP_STATUS:-500}"
  ;;
esac
EOF
chmod +x "${contract_tmp}/bin/curl" "${contract_tmp}/bootstrap-admin.sh"
printf '%s' 'contract-setup-token-at-least-32-bytes' > "${contract_tmp}/secrets/setup-token"
printf '%s' 'contract-password-at-least-12' > "${contract_tmp}/secrets/admin-password"
chmod 600 "${contract_tmp}/secrets/setup-token" "${contract_tmp}/secrets/admin-password"
bootstrap_output="${contract_tmp}/bootstrap-output"
if PATH="${contract_tmp}/bin:${PATH}" \
  BOOTSTRAP_CONTRACT_MARKER="${contract_tmp}/marker" \
  BOOTSTRAP_CONTRACT_REQUEST="${contract_tmp}/request" \
  OHC_SETUP_TOKEN_FILE="${contract_tmp}/secrets/setup-token" \
  SETUP_ADMIN_INIT_USERNAME='contract-admin' \
  SETUP_ADMIN_INIT_EMAIL='contract-admin@example.test' \
  SETUP_ADMIN_INIT_PASSWORD_FILE="${contract_tmp}/secrets/admin-password" \
  SETUP_ADMIN_INIT_ORGANIZATION_ID='contract-org' \
  sh "${contract_tmp}/bootstrap-admin.sh" >"${bootstrap_output}" 2>&1; then
  echo "Bootstrap helper must fail on an unexpected HTTP response." >&2
  exit 1
fi
if [[ -e "${contract_tmp}/marker" ]]; then
  echo "Bootstrap helper wrote its marker after an unexpected HTTP response." >&2
  exit 1
fi
if grep -Fq 'contract-setup-token-at-least-32-bytes' "$bootstrap_output" || \
   grep -Fq 'contract-password-at-least-12' "$bootstrap_output"; then
  echo "Bootstrap helper must not log setup credentials." >&2
  exit 1
fi

# A 409 only proves that some admin already exists. The helper must verify the
# configured credentials rather than silently accepting an unrelated/restored
# database identity.
if PATH="${contract_tmp}/bin:${PATH}" \
  BOOTSTRAP_CONTRACT_REQUEST="${contract_tmp}/request" \
  BOOTSTRAP_CONTRACT_SETUP_STATUS=409 \
  BOOTSTRAP_CONTRACT_LOGIN_STATUS=401 \
  OHC_SETUP_TOKEN_FILE="${contract_tmp}/secrets/setup-token" \
  SETUP_ADMIN_INIT_USERNAME='contract-admin' \
  SETUP_ADMIN_INIT_EMAIL='contract-admin@example.test' \
  SETUP_ADMIN_INIT_PASSWORD_FILE="${contract_tmp}/secrets/admin-password" \
  SETUP_ADMIN_INIT_ORGANIZATION_ID='contract-org' \
  sh "${contract_tmp}/bootstrap-admin.sh" >"${bootstrap_output}" 2>&1; then
  echo "Bootstrap helper must fail when a 409 admin cannot authenticate with the configured credentials." >&2
  exit 1
fi
if ! PATH="${contract_tmp}/bin:${PATH}" \
  BOOTSTRAP_CONTRACT_REQUEST="${contract_tmp}/request" \
  BOOTSTRAP_CONTRACT_SETUP_STATUS=409 \
  BOOTSTRAP_CONTRACT_LOGIN_STATUS=200 \
  OHC_SETUP_TOKEN_FILE="${contract_tmp}/secrets/setup-token" \
  SETUP_ADMIN_INIT_USERNAME='contract-admin' \
  SETUP_ADMIN_INIT_EMAIL='contract-admin@example.test' \
  SETUP_ADMIN_INIT_PASSWORD_FILE="${contract_tmp}/secrets/admin-password" \
  SETUP_ADMIN_INIT_ORGANIZATION_ID='contract-org' \
  sh "${contract_tmp}/bootstrap-admin.sh" >"${bootstrap_output}" 2>&1; then
  echo "Bootstrap helper must accept a 409 only after the configured admin authenticates." >&2
  exit 1
fi
require_literal '/api/v1/auth/login' "${contract_tmp}/request" \
  "Bootstrap helper did not perform the required login verification after a 409."

# Both real smoke scripts must bootstrap, log in, parse a nonempty JWT, prove
# denial without valid credentials, and authenticate protected API requests.
for script in "$kind_script" "$compose_script"; do
  require_literal '/api/v1/setup/admin' "$script" \
    "${script} must create its tenant/admin through the setup endpoint."
  require_literal '/api/v1/auth/login' "$script" \
    "${script} must perform a real login."
  require_literal 'organization_id' "$script" \
    "${script} login must include organization_id."
  require_literal "jq -er" "$script" \
    "${script} must parse its JWT as structured JSON."
  require_literal 'Authorization: Bearer ${access_token}' "$script" \
    "${script} must authenticate protected API calls with the JWT."
  require_literal 'wrong-setup-token' "$script" \
    "${script} must prove the setup endpoint rejects a wrong token."
  require_literal 'wrong-jwt' "$script" \
    "${script} must prove protected APIs reject a wrong JWT."
  require_literal '--connect-timeout' "$script" \
    "${script} must bound connection attempts."
  require_literal '--max-time' "$script" \
    "${script} must bound complete HTTP requests."
  require_literal 'openssl s_client' "$script" \
    "${script} must exercise the real gRPC TLS listener."
  require_literal 'client.crt' "$script" \
    "${script} must authenticate a CA-signed gRPC client certificate."
  require_literal 'tls-rejected' "$script" \
    "${script} must use the real gRPC client to prove TLS denies clients without certificates."
  require_literal '-verify_hostname localhost' "$script" \
    "${script} must verify the gRPC server identity."
  require_literal '-alpn h2' "$script" \
    "${script} must verify gRPC HTTP/2 negotiation."
  require_literal 'grpc_mtls_probe' "$script" \
    "${script} must exercise a real intercepted gRPC request."
  require_literal 'client-no-spiffe.crt' "$script" \
    "${script} must reject a CA-signed client without a SPIFFE identity."
done

# The setup token is sourced only from a pre-existing Secret. The fixed key is
# part of the chart contract; backend.env remains the explicit override.
require_literal 'setup:' "$chart_values" \
  "Helm values must document backend.setup."
require_literal 'existingSecret:' "$chart_values" \
  "Helm values must expose backend.setup.existingSecret."
require_literal '$setupSecret' "$backend_template" \
  "Helm backend deployment must consume backend.setup.existingSecret."
require_literal 'hasKey $backendEnv "OHC_SETUP_TOKEN"' "$backend_template" \
  "backend.env.OHC_SETUP_TOKEN must override setup Secret injection."
require_literal 'name: OHC_SETUP_TOKEN' "$backend_template" \
  "Helm backend deployment must inject OHC_SETUP_TOKEN."
require_literal 'secretKeyRef:' "$backend_template" \
  "Helm backend setup token must use secretKeyRef."
require_literal 'key: token' "$backend_template" \
  "Helm backend setup Secret must use the fixed token key."
require_literal 'optional: true' "$backend_template" \
  "Deleting the one-time setup Secret must fail closed without blocking pod startup."
require_literal 'default dict .Values.backend.setup' "$backend_template" \
  "Helm setup values must remain compatible with reused older values."
require_literal 'default dict .Values.backend.grpcTls' "$backend_template" \
  "Helm gRPC TLS values must remain compatible with reused older values."
require_literal 'automountServiceAccountToken: false' "$backend_template" \
  "Backend pods must not mount an unused Kubernetes API token."
require_literal 'seccompProfile:' "$backend_template" \
  "Backend pods must use the RuntimeDefault seccomp profile."
require_literal 'defaultMode: 0440' "$backend_template" \
  "Backend TLS material must not be world-readable."
require_literal 'default dict .Values.backend.auth' "$backend_template" \
  "Helm auth values must remain compatible with reused older values."
require_literal 'backend.auth.existingSecret' "$backend_template" \
  "Cloud Helm deployments must fail closed without a JWT Secret source."
require_literal 'backend.auth.existingSecret is required in cloud mode' "$backend_template" \
  "Cloud Helm deployments must require the chart-managed JWT Secret."
require_literal 'backend.env.JWT_SECRET and backend.env.JWT_SECRET_FILE are not supported in cloud mode' "$backend_template" \
  "Cloud Helm deployments must reject inline or unmounted JWT secret sources."
require_literal 'hasKey $backendEnv "OHC_SETUP_TOKEN_FILE"' "$backend_template" \
  "Helm must not combine direct and file-backed setup token sources."
if grep -Eq '^[[:space:]]+(token|setupToken):' "$chart_values"; then
  echo "Helm values must never accept a plaintext setup token." >&2
  exit 1
fi

# Compose must pass setup inputs to both participants and exercise server-init.
require_literal 'OHC_SETUP_TOKEN_FILE:' "$compose_manifest" \
  "Compose server must read its setup token from a secret file."
require_literal 'JWT_SECRET_FILE:' "$compose_manifest" \
  "Compose server must read its JWT secret from a secret file."
require_literal 'SETUP_ADMIN_INIT_PASSWORD_FILE:' "$compose_manifest" \
  "Compose bootstrap must read its admin password from a secret file."
require_literal 'DATABASE_URL_FILE:' "$compose_manifest" \
  "Compose server must read its database URL from a secret file."
require_literal 'POSTGRES_PASSWORD_FILE:' "$compose_manifest" \
  "Compose Postgres must read its password from a secret file."
require_literal '${OHC_DOCKER_POWERSYNC_PORT:-127.0.0.1:8082}:8080' "$compose_manifest" \
  "Compose PowerSync must not collide with the backend gRPC host port."
require_literal 'SETUP_ADMIN_INIT_EMAIL:' "$compose_manifest" \
  "Compose bootstrap must receive the admin email."
require_literal 'SETUP_ADMIN_INIT_ORGANIZATION_ID:' "$compose_manifest" \
  "Compose bootstrap must receive the organization ID."
require_literal 'compose up -d postgres valkey server server-init' "$compose_script" \
  "Compose smoke must run server-init."
require_literal 'wait server-init' "$compose_script" \
  "Compose smoke must verify server-init's exit status."
require_literal '/api/v1/dev/seed' "$compose_script" \
  "Compose smoke must seed its real database through the authenticated API."
require_literal 'length > 0' "$compose_script" \
  "Compose smoke must reject empty placeholder data after seeding."
require_literal 'postgres-data:/var/lib/postgresql/data' "$compose_manifest" \
  "Compose database state must have the same lifecycle as the bootstrap marker."
reject_literal 'restart: on-failure' "$compose_manifest" \
  "A permanently invalid bootstrap configuration must not restart forever."
reject_literal ':/etc/ohc/grpc-tls:ro' "$compose_manifest" \
  "Compose must not mount the TLS directory containing the CA private key."
for key in server.crt server.key ca.crt; do
  require_literal "/etc/ohc/grpc-tls/${key}:ro" "$compose_manifest" \
    "Compose must mount only the required ${key} TLS file."
done
reject_literal '/etc/docker/daemon.json' "$compose_script" \
  "Compose E2E must not mutate the host Docker daemon configuration."
reject_literal 'systemctl restart docker' "$compose_script" \
  "Compose E2E must not restart the host Docker daemon."

# Cloud-mode Helm rendering must fail closed when no gRPC identity Secret is
# configured, while explicit standalone rendering must remain valid.
require_literal 'fail' "$backend_template" \
  "Cloud Helm deployments must fail rendering without a gRPC TLS Secret."

# Kind must protect and clean up its cluster-admin kubeconfig.
require_literal 'umask 077' "$kind_script" \
  "Kind E2E must establish a private umask before creating credentials."
require_literal 'mktemp' "$kind_script" \
  "Kind E2E must create its kubeconfig with mktemp."
require_literal 'rm -f "${KUBECONFIG' "$kind_script" \
  "Kind E2E must remove its cluster-admin kubeconfig."

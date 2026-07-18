#!/usr/bin/env bash
# Create private, local-only credentials and TLS material for Docker Compose.
set -euo pipefail
umask 077

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd -P)"
STATE_DIR="${OHC_COMPOSE_STATE_DIR:-${REPO_ROOT}/.ohc-compose}"
ENV_FILE="${STATE_DIR}/compose.env"
TLS_DIR="${STATE_DIR}/grpc-tls"

for tool in openssl sed; do
  command -v "$tool" >/dev/null 2>&1 || {
    echo "error: required tool '$tool' not found" >&2
    exit 1
  }
done

if [[ -f "${ENV_FILE}" ]]; then
  echo "Compose credentials already exist at ${ENV_FILE}."
  exit 0
fi
if [[ $# -gt 0 ]]; then
  echo "usage: $0" >&2
  exit 2
fi

mkdir -p "${STATE_DIR}" "${TLS_DIR}"
find "${TLS_DIR}" -mindepth 1 -maxdepth 1 -type f -delete

POSTGRES_PASSWORD="$(openssl rand -hex 24)"
JWT_SECRET="$(openssl rand -hex 48)"
SETUP_TOKEN="$(openssl rand -hex 32)"
ADMIN_PASSWORD="OHC-Local-Aa1-$(openssl rand -hex 18)"

printf '%s' "${POSTGRES_PASSWORD}" > "${STATE_DIR}/postgres-password"
printf '%s' "${JWT_SECRET}" > "${STATE_DIR}/jwt-secret"
printf '%s' "${SETUP_TOKEN}" > "${STATE_DIR}/setup-token"
printf '%s' "${ADMIN_PASSWORD}" > "${STATE_DIR}/admin-password"
printf 'postgres://ohc:%s@postgres:5432/ohc?sslmode=disable' \
  "${POSTGRES_PASSWORD}" > "${STATE_DIR}/database-url"

openssl req -x509 -newkey rsa:3072 -nodes -sha256 -days 365 \
  -keyout "${TLS_DIR}/ca.key" \
  -out "${TLS_DIR}/ca.crt" \
  -subj '/CN=OHC local Compose CA' \
  -addext 'basicConstraints=critical,CA:TRUE' \
  -addext 'keyUsage=critical,keyCertSign,cRLSign' >/dev/null 2>&1
openssl req -new -newkey rsa:3072 -nodes -sha256 \
  -keyout "${TLS_DIR}/server.key" \
  -out "${TLS_DIR}/server.csr" \
  -subj '/CN=localhost' >/dev/null 2>&1
printf '%s\n' \
  'basicConstraints=critical,CA:FALSE' \
  'keyUsage=critical,digitalSignature,keyEncipherment' \
  'extendedKeyUsage=serverAuth' \
  'subjectAltName=DNS:localhost,DNS:server,IP:127.0.0.1' \
  > "${TLS_DIR}/server.ext"
openssl x509 -req -sha256 -days 365 \
  -in "${TLS_DIR}/server.csr" \
  -CA "${TLS_DIR}/ca.crt" \
  -CAkey "${TLS_DIR}/ca.key" \
  -CAcreateserial \
  -out "${TLS_DIR}/server.crt" \
  -extfile "${TLS_DIR}/server.ext" >/dev/null 2>&1
find "${TLS_DIR}" -mindepth 1 -maxdepth 1 -type f \
  ! -name 'ca.crt' ! -name 'ca.key' ! -name 'server.crt' ! -name 'server.key' -delete

escape_env_value() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

cat > "${ENV_FILE}" <<EOF
OHC_DOCKER_GRPC_TLS_DIR="$(escape_env_value "${TLS_DIR}")"
JWT_SECRET_FILE="$(escape_env_value "${STATE_DIR}/jwt-secret")"
OHC_SETUP_TOKEN_FILE="$(escape_env_value "${STATE_DIR}/setup-token")"
SETUP_ADMIN_INIT_PASSWORD_FILE="$(escape_env_value "${STATE_DIR}/admin-password")"
OHC_POSTGRES_PASSWORD_FILE="$(escape_env_value "${STATE_DIR}/postgres-password")"
DATABASE_URL_FILE="$(escape_env_value "${STATE_DIR}/database-url")"
SETUP_ADMIN_INIT_USERNAME="$(escape_env_value "${SETUP_ADMIN_INIT_USERNAME:-admin}")"
SETUP_ADMIN_INIT_EMAIL="$(escape_env_value "${SETUP_ADMIN_INIT_EMAIL:-admin@example.test}")"
SETUP_ADMIN_INIT_ORGANIZATION_ID="$(escape_env_value "${SETUP_ADMIN_INIT_ORGANIZATION_ID:-local}")"
OHC_DOCKER_UID="$(id -u)"
OHC_DOCKER_GID="$(id -g)"
EOF
chmod 700 "${STATE_DIR}" "${TLS_DIR}"
find "${STATE_DIR}" -type f -exec chmod 600 {} +

echo "Created private Compose configuration: ${ENV_FILE}"
echo "Initial admin password is stored at: ${STATE_DIR}/admin-password"
echo "Start with: docker compose --env-file ${ENV_FILE} -f deploy/docker-compose.yml -f deploy/docker-compose.override.yml up -d"

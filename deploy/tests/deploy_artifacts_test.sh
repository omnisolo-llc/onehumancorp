#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

compose_file="${root}/deploy/docker-compose.yml"
chart_file="${root}/deploy/helm/omnisolo/Chart.yaml"
values_file="${root}/deploy/helm/omnisolo/values.yaml"
toolchain_file="${root}/rust-toolchain.toml"
bootstrap_file="${root}/deploy/docker/server-init/bootstrap-admin.sh"
standalone_file="${root}/deploy/scripts/omnisolo-standalone.sh"
server_dockerfile="${root}/deploy/docker/server/Dockerfile"
web_dockerfile="${root}/deploy/docker/web/Dockerfile"
web_package_file="${root}/src/ui/next/package.json"
dockerignore_file="${root}/.dockerignore"

# Verify required deployment files are present and non-empty.
for file in \
  "$compose_file" \
  "$chart_file" \
  "$values_file" \
  "$toolchain_file" \
  "$bootstrap_file" \
  "$dockerignore_file" \
  "$server_dockerfile" \
  "$web_dockerfile" \
  "$web_package_file" \
  "$standalone_file"; do
  test -s "$file"
done

# Native target-platform builds preserve SQLCipher's OpenSSL headers/runtime.
# Do not pin builder stages to BUILDPLATFORM: that would ship host-architecture
# Rust/Node binaries in an ARM64 image. Buildx selects the target for all stages.
grep -q 'libssl-dev' "$server_dockerfile"
grep -q '^FROM rust:' "$server_dockerfile"
! grep -q 'FROM --platform=\$BUILDPLATFORM' "$server_dockerfile"
grep -q 'cargo chef cook --release --locked' "$server_dockerfile"
grep -q 'cargo build --locked --release' "$server_dockerfile"
grep -q 'libssl3' "$server_dockerfile"
grep -q 'npm ci' "$web_dockerfile"
grep -q '^FROM node:' "$web_dockerfile"
! grep -q 'FROM --platform=\$BUILDPLATFORM' "$web_dockerfile"
grep -q 'npm run build:web' "$web_dockerfile"
grep -q 'COPY --from=builder --chown=node:node /src/target/native-web' "$web_dockerfile"
grep -q '^USER node' "$web_dockerfile"
grep -Fq 'CMD ["node", "src/ui/next/server.js"]' "$web_dockerfile"
python3 - "$web_package_file" <<'PY'
import json
import pathlib
import sys

package = json.loads(pathlib.Path(sys.argv[1]).read_text())
if "next" not in package.get("dependencies", {}):
    raise SystemExit("Next.js must be a runtime dependency for the production web image")
if "next" in package.get("devDependencies", {}):
    raise SystemExit("Next.js must not remain a dev-only dependency in the production web image")
PY
grep -qx "target" "$dockerignore_file"
grep -qx "node_modules" "$dockerignore_file"

# All required native executables and migrations must be included, with an
# unprivileged runtime and independently selectable server/agent/worker images.
for binary in server omnisolo-builtin-agent omnisolo-harness-worker; do
  grep -Fq "/src/target/release/$binary /usr/local/bin/$binary" "$server_dockerfile"
done
for role in server agent worker; do
  grep -q "^FROM runtime AS $role$" "$server_dockerfile"
done
grep -q '^USER omnisolo$' "$server_dockerfile"
grep -q '/src/src/server/migrations /src/server/migrations' "$server_dockerfile"
grep -q '/src/src/server/db/migrations /src/server/db/migrations' "$server_dockerfile"
# Dependency layers must be reusable; workspace source is copied only afterward.
python3 - "$server_dockerfile" <<'PY'
import pathlib, sys
text = pathlib.Path(sys.argv[1]).read_text()
builder = text.split('FROM chef AS builder', 1)[1]
assert builder.index('cargo chef cook') < builder.index('COPY . .') < builder.index('cargo build'), 'Dependency-layer order is broken'
PY

# Verify docker-compose uses the consolidated server image.
grep -q "server:" "$compose_file"
grep -q "omnisolo/server:latest" "$compose_file"
! grep -q "omnisolo/ui" "$compose_file"
! grep -q "^  ui:" "$compose_file"

grep -q "backend" "$values_file"
grep -q "valkey" "$values_file"

grep -q "Deployment" "${root}/deploy/helm/omnisolo/templates/backend-deployment.yaml"
test ! -e "${root}/deploy/helm/omnisolo/templates/frontend-deployment.yaml"
test ! -e "${root}/deploy/helm/omnisolo/templates/frontend-service.yaml"

# Verify health probes are wired in the backend deployment template.
grep -q "livenessProbe" "${root}/deploy/helm/omnisolo/templates/backend-deployment.yaml"
grep -q "readinessProbe" "${root}/deploy/helm/omnisolo/templates/backend-deployment.yaml"

# Verify deploy startup scripts use the readiness endpoint exposed by src/server/lib.rs.
grep -q "/readyz" "$bootstrap_file"
grep -q "/readyz" "$standalone_file"
! grep -q "/health " "$bootstrap_file"
! grep -q "/health " "$standalone_file"

echo "deployment artifact checks passed"

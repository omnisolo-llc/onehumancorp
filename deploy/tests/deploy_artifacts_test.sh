#!/usr/bin/env bash
set -euo pipefail

repo_name="${TEST_WORKSPACE:-mono}"
root="${TEST_SRCDIR}/${repo_name}"

compose_file="${root}/deploy/docker-compose.yml"
chart_file="${root}/deploy/helm/omnisolo/Chart.yaml"
values_file="${root}/deploy/helm/omnisolo/values.yaml"
build_file="${root}/deploy/BUILD.bazel"
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
  "$build_file" \
  "$bootstrap_file" \
  "$dockerignore_file" \
  "$server_dockerfile" \
  "$web_dockerfile" \
  "$web_package_file" \
  "$standalone_file"; do
  test -s "$file"
done

# The native ARM64 image path must keep SQLCipher's build-time OpenSSL headers
# and the matching runtime library available.
grep -q "libssl-dev" "$server_dockerfile"
grep -q 'FROM --platform=\$BUILDPLATFORM rust:' "$server_dockerfile"
grep -q "gcc-aarch64-linux-gnu" "$server_dockerfile"
grep -q "rustup target add aarch64-unknown-linux-gnu" "$server_dockerfile"
grep -q "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER" "$server_dockerfile"
grep -q "cargo build --locked --release --target aarch64-unknown-linux-gnu --bin server" "$server_dockerfile"
grep -q "libssl3" "$server_dockerfile"
grep -q "npm ci" "$web_dockerfile"
grep -q 'FROM --platform=\$BUILDPLATFORM node:' "$web_dockerfile"
grep -q "npm run build" "$web_dockerfile"
grep -q "COPY --from=builder --chown=node:node /app/.next" "$web_dockerfile"
grep -q "USER node" "$web_dockerfile"
grep -q 'CMD \["npm", "run", "start"\]' "$web_dockerfile"
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

# Verify OCI bazel rules are present (Dockerfiles replaced by rules_oci).
grep -q "oci_image" "$build_file"
grep -q "server_image" "$build_file"
grep -q "default_agent_image" "$build_file"
grep -q "omnisolo-builtin-agent" "$build_file"
grep -q "distroless" "$build_file"
grep -q "internal-default-agent:bazel" "$build_file"

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

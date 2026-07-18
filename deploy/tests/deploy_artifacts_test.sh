#!/usr/bin/env bash
set -euo pipefail

repo_name="${TEST_WORKSPACE:-}"
root="${TEST_SRCDIR}/${repo_name}"
if [ -z "$repo_name" ]; then root="${TEST_SRCDIR}"; fi

compose_file="${root}/deploy/docker-compose.yml"
chart_file="${root}/deploy/helm/ohc/Chart.yaml"
values_file="${root}/deploy/helm/ohc/values.yaml"
build_file="${root}/deploy/BUILD.bazel"
bootstrap_file="${root}/deploy/docker/server-init/bootstrap-admin.sh"
standalone_file="${root}/deploy/scripts/ohc-standalone.sh"

# Verify required deployment files are present and non-empty.
for file in \
  "$compose_file" \
  "$chart_file" \
  "$values_file" \
  "$build_file" \
  "$bootstrap_file" \
  "$standalone_file"; do
  test -s "$file"
done

# Verify OCI bazel rules are present (Dockerfiles replaced by rules_oci).
grep -q "oci_image" "$build_file"
grep -q "server_image" "$build_file"
grep -q "default_agent_image" "$build_file"
grep -q "ohc-builtin-agent" "$build_file"
grep -q "distroless" "$build_file"
grep -q "internal-default-agent:bazel" "$build_file"

# Verify docker-compose uses the consolidated server image.
grep -q "server:" "$compose_file"
grep -q "onehumancorp/server:latest" "$compose_file"
! grep -q "onehumancorp/ui" "$compose_file"
! grep -q "^  ui:" "$compose_file"

grep -q "backend" "$values_file"
grep -q "valkey" "$values_file"

grep -q "Deployment" "${root}/deploy/helm/ohc/templates/backend-deployment.yaml"
test ! -e "${root}/deploy/helm/ohc/templates/frontend-deployment.yaml"
test ! -e "${root}/deploy/helm/ohc/templates/frontend-service.yaml"

# Verify health probes are wired in the backend deployment template.
grep -q "livenessProbe" "${root}/deploy/helm/ohc/templates/backend-deployment.yaml"
grep -q "readinessProbe" "${root}/deploy/helm/ohc/templates/backend-deployment.yaml"

# Verify deploy startup scripts use the readiness endpoint exposed by src/server/lib.rs.
grep -q "/readyz" "$bootstrap_file"
grep -q "/readyz" "$standalone_file"
! grep -q "/health " "$bootstrap_file"
! grep -q "/health " "$standalone_file"

# BusyBox wget appends an error summary for non-2xx responses; the parser must
# only read actual HTTP status lines so it does not report "server" as a status.
grep -Fq '^[[:space:]]*HTTP\/' "$bootstrap_file"

echo "deployment artifact checks passed"

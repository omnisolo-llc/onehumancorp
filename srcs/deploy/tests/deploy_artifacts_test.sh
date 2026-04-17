#!/usr/bin/env bash
set -euo pipefail

repo_name="${TEST_WORKSPACE:-mono}"
root="${TEST_SRCDIR}/${repo_name}"

compose_file="${root}/srcs/deploy/docker-compose.yml"
chart_file="${root}/srcs/deploy/helm/ohc/Chart.yaml"
values_file="${root}/srcs/deploy/helm/ohc/values.yaml"
build_file="${root}/srcs/deploy/BUILD.bazel"

# Verify required deployment files are present and non-empty.
for file in \
  "$compose_file" \
  "$chart_file" \
  "$values_file" \
  "$build_file"; do
  test -s "$file"
done

# Verify OCI bazel rules are present (Dockerfiles replaced by rules_oci).
grep -q "oci_image" "$build_file"
grep -q "server_image" "$build_file"
grep -q "default_agent_image" "$build_file"
grep -q "distroless" "$build_file"
grep -q "internal-default-agent:bazel" "$build_file"

# Verify docker-compose uses the consolidated server image.
grep -q "server:" "$compose_file"
grep -q "onehumancorp/server:latest" "$compose_file"
! grep -q "onehumancorp/ui" "$compose_file"
! grep -q "^  ui:" "$compose_file"

grep -q "backend" "$values_file"
grep -q "redis" "$values_file"

grep -q "Deployment" "${root}/srcs/deploy/helm/ohc/templates/backend-deployment.yaml"
test ! -e "${root}/srcs/deploy/helm/ohc/templates/frontend-deployment.yaml"
test ! -e "${root}/srcs/deploy/helm/ohc/templates/frontend-service.yaml"

# Verify health probes are wired in the backend deployment template.
grep -q "livenessProbe" "${root}/srcs/deploy/helm/ohc/templates/backend-deployment.yaml"
grep -q "readinessProbe" "${root}/srcs/deploy/helm/ohc/templates/backend-deployment.yaml"

echo "deployment artifact checks passed"

#!/bin/bash
set -e

# Sync all dashboards from monitoring/dashboards/ to deploy/helm/ohc/dashboards/ and deploy/docker/grafana/provisioning/dashboards/
echo "Syncing dashboards..."

SRC_DIR="monitoring/dashboards"
HELM_DIR="deploy/helm/ohc/dashboards"
DOCKER_DIR="deploy/docker/grafana/provisioning/dashboards"

# Ensure target directories exist
mkdir -p "$HELM_DIR"
mkdir -p "$DOCKER_DIR"

# Copy all JSON files from source to both targets
for file in "$SRC_DIR"/*.json; do
  filename=$(basename "$file")
  cp "$file" "$HELM_DIR/$filename"
  cp "$file" "$DOCKER_DIR/$filename"
done

echo "Sync complete."

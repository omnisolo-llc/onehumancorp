#!/bin/bash
set -e

# Sync all json dashboards from src/server/monitoring/dashboards to deploy folders
# It assumes it's running from the root of the workspace.
for db in src/server/monitoring/dashboards/*.json; do
  filename=$(basename "$db")
  cp "$db" "deploy/docker/grafana/provisioning/dashboards/$filename"
  cp "$db" "deploy/helm/ohc/dashboards/$filename"
  echo "Synced $filename"
done

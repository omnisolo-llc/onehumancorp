#!/bin/bash
set -e

echo "Updating Grafana Dashboards..."

# Source directory
SRC_DIR="deploy/grafana/dashboards"

# Destination directories
DOCKER_DEST="deploy/docker/grafana/provisioning/dashboards"
HELM_DEST="deploy/helm/ohc/dashboards"

mkdir -p "$DOCKER_DEST"
mkdir -p "$HELM_DEST"

cp $SRC_DIR/*.json "$DOCKER_DEST/"
cp $SRC_DIR/*.json "$HELM_DEST/"

echo "Dashboards updated successfully."

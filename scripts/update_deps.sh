#!/usr/bin/env bash
set -euo pipefail

# Script to refresh Bazel and cargo dependencies
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Running bazel mod tidy..."
bazel mod tidy

echo "Dependencies up to date."

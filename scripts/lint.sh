#!/usr/bin/env bash
set -euo pipefail

# If run under `bazel run`, change back to the workspace root directory
if [ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]; then
  cd "$BUILD_WORKSPACE_DIRECTORY"
fi

echo "Running Clippy aspect on all Rust targets..."
exec bazelisk build \
  --keep_going \
  --aspects=@rules_rust//rust:defs.bzl%rust_clippy_aspect \
  --output_groups=clippy_checks \
  //src/... "$@"

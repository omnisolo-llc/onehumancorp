#!/usr/bin/env bash
# Run cargo tests for the ohc-builtin-agent.
# This script is used as the srcs of a Bazel sh_test target so that
# `bazelisk test //...` exercises the Rust crate alongside the Go tests.
set -euo pipefail

# Resolve the real source directory by following symlinks.
# In a Bazel execroot the script file is a symlink into the workspace;
# readlink -f gives us the actual on-disk path where Cargo.toml lives.
REAL_SCRIPT="$(readlink -f "${BASH_SOURCE[0]}")"
SCRIPT_DIR="$(dirname "$REAL_SCRIPT")"

cd "$SCRIPT_DIR"
exec cargo test

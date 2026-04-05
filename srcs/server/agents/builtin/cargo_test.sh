#!/usr/bin/env bash
# Run cargo tests for the ohc-builtin-agent.
# This script is used as the srcs of a Bazel sh_test target so that
# `bazelisk test //...` exercises the Rust crate alongside the Go tests.
set -euo pipefail

# Resolve the real source directory by following symlinks portably.
# In a Bazel execroot the script file is a symlink into the workspace;
# resolving it gives us the actual on-disk path where Cargo.toml lives.
# (readlink -f is GNU-only; this loop works on both macOS and Linux.)
SCRIPT="${BASH_SOURCE[0]}"
while [[ -L "$SCRIPT" ]]; do
    SCRIPT="$(readlink "$SCRIPT")"
done
SCRIPT_DIR="$(cd "$(dirname "$SCRIPT")" && pwd)"

cd "$SCRIPT_DIR"
exec cargo test

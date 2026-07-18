#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${TEST_SRCDIR:-}" ]]; then
  echo "api_versioning_docs_test must run under Bazel" >&2
  exit 1
fi

node_bin="$(find "$TEST_SRCDIR" -path '*/bin/node' -executable | head -n 1)"
if [[ -z "$node_bin" ]]; then node_bin="$(which node)"; fi
if [[ -z "$node_bin" ]]; then
  echo "missing Bazel-provided Node.js binary" >&2
  exit 1
fi

"$node_bin" --test "$TEST_SRCDIR/${TEST_WORKSPACE:-}/scripts/api-versioning-docs.test.mjs"

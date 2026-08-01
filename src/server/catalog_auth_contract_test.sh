#!/usr/bin/env bash
set -euo pipefail

server_source="${OHC_SERVER_LIB_SOURCE:-${TEST_SRCDIR:-}/${TEST_WORKSPACE:-_main}/src/server/lib.rs}"
if [[ ! -f "$server_source" ]]; then
  server_source="$(git rev-parse --show-toplevel)/src/server/lib.rs"
fi

if command -v rg >/dev/null 2>&1; then
  rg -U -q '"/api/v1/catalog",\s*api::catalog::router\([\s\S]*?strict_bearer_auth_middleware' "$server_source"
else
  # Use perl to simulate multiline match if rg is missing (which happens in bazel linux-sandbox CI if ripgrep isn't a declared toolchain)
  perl -0777 -ne 'exit(!m{"/api/v1/catalog",\s*api::catalog::router\([^)]*?strict_bearer_auth_middleware})' "$server_source"
fi

#!/usr/bin/env bash
set -euo pipefail

server_source="${OHC_SERVER_LIB_SOURCE:-${TEST_SRCDIR:-}/${TEST_WORKSPACE:-_main}/src/server/lib.rs}"
if [[ ! -f "$server_source" ]]; then
  server_source="$(git rev-parse --show-toplevel)/src/server/lib.rs"
fi

grep -q -Pzo '"/api/v1/catalog",\s*api::catalog::router\([\s\S]*?strict_bearer_auth_middleware' "$server_source"

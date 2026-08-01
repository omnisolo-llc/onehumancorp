#!/usr/bin/env bash
set -euo pipefail

server_source="${OHC_SERVER_LIB_SOURCE:-${TEST_SRCDIR:-}/${TEST_WORKSPACE:-_main}/src/server/lib.rs}"
if [[ ! -f "$server_source" ]]; then
  server_source="$(git rev-parse --show-toplevel)/src/server/lib.rs"
fi

python3 - "$server_source" <<'PY'
import pathlib
import re
import sys

source = pathlib.Path(sys.argv[1]).read_text()
pattern = r'"/api/v1/catalog",\s*api::catalog::router\([\s\S]*?strict_bearer_auth_middleware'
if not re.search(pattern, source):
    raise SystemExit("catalog router is not protected by strict bearer authentication")
PY

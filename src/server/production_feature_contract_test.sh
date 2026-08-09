#!/usr/bin/env bash
set -euo pipefail

source_root="${TEST_SRCDIR:-}/${TEST_WORKSPACE:-_main}/src/server"
if [[ ! -f "$source_root/lib.rs" ]]; then
  source_root="$(git rev-parse --show-toplevel)/src/server"
fi

server_source="$source_root/lib.rs"
catalog_source="$source_root/api/catalog.rs"

[[ -f "$server_source" ]] || { echo "server source is unavailable: $server_source" >&2; exit 1; }
[[ -f "$catalog_source" ]] || { echo "catalog source is unavailable: $catalog_source" >&2; exit 1; }

python3 - "$server_source" "$catalog_source" <<'PY'
import pathlib
import re
import sys

server = pathlib.Path(sys.argv[1]).read_text()
catalog = pathlib.Path(sys.argv[2]).read_text()

if "legacy_db_compatibility_layer" not in server:
    raise SystemExit("legacy database routes do not use a shared compatibility layer")

if "Extension(std::sync::Arc::new(db.clone()))" in server:
    raise SystemExit("legacy database layer still wraps an Arc<DB> in a nested Arc")

for route in ("/api/v1/help", "/api/v1/tooltips", "/api/v1/videos"):
    if route not in server:
        raise SystemExit(f"legacy route is missing from the server router: {route}")

for fabricated in ("Generated Offering", "AI description", '"10.00"'):
    if fabricated in catalog:
        raise SystemExit(f"catalog generation still contains fabricated fallback data: {fabricated}")

if not re.search(r"MINIMAX_API_KEY", catalog):
    raise SystemExit("catalog generation does not declare its real provider dependency")
PY

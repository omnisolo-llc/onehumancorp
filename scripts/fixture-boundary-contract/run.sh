#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/fixture-boundary-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/fixture-boundary-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/fixture-boundary-contract/verify_lock.py
python3 scripts/fixture-boundary-contract/prepare.py
before=$(sha256sum scripts/fixture-boundary-contract/source-manifest.json)
log=$(mktemp)
trap 'rm -f "$log"' EXIT
status=0
cargo test --locked --offline --manifest-path scripts/fixture-boundary-contract/Cargo.toml -- --test-threads=1 2>&1 | tee "$log" || status=$?
python3 - "$log" <<'PY' || status=$?
from pathlib import Path
import re, sys
results = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', Path(sys.argv[1]).read_text())
if not results or sum(int(item[0]) for item in results) < 1 or any(any(int(value) for value in item[1:]) for item in results):
    raise SystemExit('Fixture boundary requires the actual middleware HTTP test, with no failures, ignored or filtered tests')
PY
python3 scripts/fixture-boundary-contract/prepare.py
test "$before" = "$(sha256sum scripts/fixture-boundary-contract/source-manifest.json)"
exit "$status"

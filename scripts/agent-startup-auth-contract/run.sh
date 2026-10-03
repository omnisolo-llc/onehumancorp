#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/agent-startup-auth-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/agent-startup-auth-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/agent-startup-auth-contract/verify_lock.py
python3 scripts/agent-startup-auth-contract/prepare.py
before=$(sha256sum scripts/agent-startup-auth-contract/source-manifest.json)
log=$(mktemp)
trap 'rm -f "$log"' EXIT
status=0
cargo test --locked --offline --manifest-path scripts/agent-startup-auth-contract/Cargo.toml --test startup -- --test-threads=1 2>&1 | tee "$log" || status=$?
python3 - "$log" <<'PY' || status=$?
from pathlib import Path
import re, sys
source = Path(sys.argv[1]).read_text()
results = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', source)
if not results or sum(int(item[0]) for item in results) < 7 or any(any(int(value) for value in item[1:]) for item in results):
    raise SystemExit('The startup authentication gate requires all seven non-test startup subprocess cases, with no failures, ignored or filtered tests')
PY
python3 scripts/agent-startup-auth-contract/prepare.py
test "$before" = "$(sha256sum scripts/agent-startup-auth-contract/source-manifest.json)"
exit "$status"

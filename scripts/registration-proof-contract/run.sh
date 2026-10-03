#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export TEST_WORKSPACE="$PWD"
export TEST_TMPDIR="$(mktemp -d /tmp/ohc-registration-proof.XXXXXX)"
trap 'rm -rf -- "$TEST_TMPDIR"' EXIT
python3 scripts/registration-proof-contract/prepare.py
cp Cargo.lock scripts/registration-proof-contract/.build/Cargo.lock
cargo metadata --offline --manifest-path scripts/registration-proof-contract/.build/Cargo.toml --format-version 1 >/dev/null
python3 - <<'PY'
from pathlib import Path
import tomllib
root = tomllib.loads(Path('Cargo.lock').read_text())['package']
focused = tomllib.loads(Path('scripts/registration-proof-contract/.build/Cargo.lock').read_text())['package']
keys = {(p['name'], p['version'], p.get('source'), p.get('checksum')) for p in root if p.get('source')}
for p in focused:
    if p.get('source') and (p['name'], p['version'], p.get('source'), p.get('checksum')) not in keys:
        raise SystemExit(f"Focused auth dependency differs from root lock: {p['name']} {p['version']}")
print('Focused auth registry identities match root Cargo.lock')
PY
before=$(sha256sum scripts/registration-proof-contract/source-manifest.json)
set +e
cargo test --locked --offline --manifest-path scripts/registration-proof-contract/.build/Cargo.toml "$@" 2>&1 | tee "$TEST_TMPDIR/result.log"
results=("${PIPESTATUS[@]}")
set -e
status=${results[0]}
if (( status == 0 && results[1] != 0 )); then status=${results[1]}; fi
if (( status == 0 )); then
  python3 - "$TEST_TMPDIR/result.log" <<'COUNTS'
from pathlib import Path
import re,sys
text=Path(sys.argv[1]).read_text()
counts=re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", text)
if not counts or sum(int(passed) for passed,_,_ in counts)==0 or any(int(failed) or int(ignored) for _,failed,ignored in counts):
    raise SystemExit("Registration gate requires nonzero executed tests and zero failed/ignored tests")
print(f"Verified {sum(int(passed) for passed,_,_ in counts)} executed tests and zero ignored tests")
COUNTS
fi
python3 scripts/registration-proof-contract/prepare.py
test "$before" = "$(sha256sum scripts/registration-proof-contract/source-manifest.json)"
exit "$status"

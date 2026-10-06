#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
: "${OHC_INVENTORY_TEST_DATABASE_URL:?Supply an isolated loopback ohc_*_test PostgreSQL database}"
mkdir -p target/ci-logs
fingerprint() {
  sha256sum src/server/api/pos.rs src/server/api/pos_inventory.rs src/server/api/pos_inventory_test.rs src/server/migrations/1041_inventory_adjustment_receipts.sql scripts/run-inventory-postgres.sh
}
fingerprint > target/ci-logs/inventory-source-before.txt
cargo test --locked -p omnisolo --lib api::pos::inventory::tests::postgres_ -- --ignored --test-threads=2 2>&1 | tee target/ci-logs/inventory-postgres.log
fingerprint > target/ci-logs/inventory-source-after.txt
cmp target/ci-logs/inventory-source-before.txt target/ci-logs/inventory-source-after.txt
python3 - <<'PY'
import pathlib,re
source=pathlib.Path('src/server/api/pos_inventory_test.rs').read_text()
expected=re.findall(r'async fn (postgres_[a-z0-9_]+)\(',source)
assert len(expected)>=11 and len(expected)==len(set(expected)), 'PostgreSQL regression inventory disappeared'
log=pathlib.Path('target/ci-logs/inventory-postgres.log').read_text()
executed=re.findall(r'^test api::pos::inventory::tests::(postgres_[a-z0-9_]+) \.\.\. ok$',log,re.M)
assert sorted(executed)==sorted(expected), f'Expected {expected}, got {executed}'
assert re.search(r'test result: ok\. '+str(len(expected))+r' passed; 0 failed; 0 ignored;',log), 'Missing successful actual execution summary'
print(f'Verified {len(expected)} executed PostgreSQL inventory contracts against the unchanged source fingerprint')
PY

#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
# The checked-in lock preserves the tested cache graph. Never resolve it afresh.
python3 scripts/staff-timecard-contract/verify_lock.py
clock_host=$(rustc -vV | sed -n 's/^host: //p')
test -n "$clock_host"
cargo fetch --locked --manifest-path scripts/staff-timecard-contract/Cargo.toml --target "$clock_host"

#!/usr/bin/env bash
# Run the real decision-handler tests with an owned disposable local database.
set -euo pipefail
cd "$(dirname "$0")/../.."
fixture=$(mktemp -d "${TMPDIR:-/tmp}/ohc-approval-read.XXXXXX")
port=${OHC_APPROVAL_TEST_PORT:-55491}
printf 'public-owned-approval-test-password\n' > "$fixture/password"
initdb -D "$fixture/data" -U approval_fixture --pwfile="$fixture/password" --auth-local=trust --auth-host=scram-sha-256 --encoding=UTF8 --no-locale > "$fixture/init.log"
trap 'pg_ctl -D "$fixture/data" -m fast -w stop > "$fixture/stop.log" 2>&1 || true' EXIT
pg_ctl -D "$fixture/data" -l "$fixture/server.log" -o "-h 127.0.0.1 -p $port -c unix_socket_directories= -c timezone=UTC -c log_timezone=UTC" -w start
export PGPASSWORD=public-owned-approval-test-password
createdb -h 127.0.0.1 -p "$port" -U approval_fixture ohc_approval_test
export OHC_APPROVAL_TEST_DATABASE_URL="postgres://approval_fixture:public-owned-approval-test-password@127.0.0.1:$port/ohc_approval_test"
if (( $# )); then
    "$@"
else
    cargo test --locked -p omnisolo --lib api::agents::approvals::readback_tests -- --test-threads=1
fi

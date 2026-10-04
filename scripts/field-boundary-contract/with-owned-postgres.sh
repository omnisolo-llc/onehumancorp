#!/usr/bin/env bash
# This wrapper keeps database and checks in one retained executor namespace.
set -euo pipefail
cd "$(dirname "$0")/../.."
root=$(mktemp -d /tmp/ohc-field-contract.XXXXXX)
port=${OHC_FIELD_TEST_PORT:-55436}
printf 'public-owned-field-test-password\n' > "$root/password"
initdb -D "$root/data" -U field_fixture --pwfile="$root/password" --auth-local=trust --auth-host=scram-sha-256 --encoding=UTF8 --no-locale > "$root/init.log"
trap 'pg_ctl -D "$root/data" -m fast -w stop > "$root/stop.log" 2>&1 || true' EXIT
pg_ctl -D "$root/data" -l "$root/server.log" -o "-h 127.0.0.1 -p $port -c unix_socket_directories= -c timezone=UTC -c log_timezone=UTC" -w start
export PGPASSWORD=public-owned-field-test-password
createdb -h 127.0.0.1 -p "$port" -U field_fixture ohc_field_test
export OHC_FIELD_TEST_DATABASE_URL="postgres://field_fixture:public-owned-field-test-password@127.0.0.1:$port/ohc_field_test"
scripts/field-boundary-contract/run.sh "$@"

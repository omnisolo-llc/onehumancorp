#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
fixture=$(mktemp -d "${TMPDIR:-/tmp}/ohc-cash-receipt.XXXXXX")
port=${OHC_CASH_TEST_PORT:-55461}
redis_port=${OHC_CASH_TEST_REDIS_PORT:-55462}
printf 'public-owned-cash-test-password\n' > "$fixture/password"
initdb -D "$fixture/data" -U cash_fixture --pwfile="$fixture/password" --auth-local=trust --auth-host=scram-sha-256 --encoding=UTF8 --no-locale > "$fixture/init.log"
redis-server --bind 127.0.0.1 --port "$redis_port" --save '' --appendonly no --daemonize no --dir "$fixture" > "$fixture/redis.log" 2>&1 &
redis_pid=$!
trap 'kill "$redis_pid" 2>/dev/null || true; pg_ctl -D "$fixture/data" -m fast -w stop > "$fixture/stop.log" 2>&1 || true; printf "Owned fixture stopped: %s\n" "$fixture"' EXIT
pg_ctl -D "$fixture/data" -l "$fixture/server.log" -o "-h 127.0.0.1 -p $port -c unix_socket_directories= -c timezone=UTC -c log_timezone=UTC" -w start
export PGPASSWORD=public-owned-cash-test-password
createdb -h 127.0.0.1 -p "$port" -U cash_fixture ohc_cash_test
export OHC_CASH_TEST_DATABASE_URL="postgres://cash_fixture:public-owned-cash-test-password@127.0.0.1:$port/ohc_cash_test"
export OHC_CASH_TEST_REDIS_URL="redis://127.0.0.1:$redis_port"
export OMNISOLO_DATABASE_URL="$OHC_CASH_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
if (( $# )); then "$@"; else scripts/cash-receipts/run.sh; fi

#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
root=$(mktemp -d /tmp/ohc-feed-contract.XXXXXX)
port=${OHC_FEED_TEST_PORT:-55442}
printf 'public-owned-feed-test-password\n' > "$root/password"
initdb -D "$root/data" -U feed_fixture --pwfile="$root/password" --auth-local=trust --auth-host=scram-sha-256 --encoding=UTF8 --no-locale > "$root/init.log"
redis-server --bind 127.0.0.1 --port 55444 --save '' --appendonly no --daemonize no --dir "$root" > "$root/redis.log" 2>&1 &
redis_pid=$!
export OHC_FEED_TEST_REDIS_URL=redis://127.0.0.1:55444
trap 'kill "$redis_pid" 2>/dev/null || true; pg_ctl -D "$root/data" -m fast -w stop > "$root/stop.log" 2>&1 || true' EXIT
pg_ctl -D "$root/data" -l "$root/server.log" -o "-h 127.0.0.1 -p $port -c unix_socket_directories= -c timezone=UTC -c log_timezone=UTC" -w start
export PGPASSWORD=public-owned-feed-test-password
createdb -h 127.0.0.1 -p "$port" -U feed_fixture ohc_feed_test
export OHC_FEED_TEST_DATABASE_URL="postgres://feed_fixture:public-owned-feed-test-password@127.0.0.1:$port/ohc_feed_test"
psql -h 127.0.0.1 -p "$port" -U feed_fixture -d ohc_feed_test -v ON_ERROR_STOP=1 -c "CREATE ROLE ohc_bypassrls BYPASSRLS"
if [[ $# -gt 0 ]]; then
  scripts/agent-feed-decision-contract/run.sh "$@"
else
  python3 scripts/focused_ci_gate.py agent-feed-decision-contract
fi

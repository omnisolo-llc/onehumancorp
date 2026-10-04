#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_FEED_TEST_DATABASE_URL:?explicit disposable PostgreSQL prerequisite required}"
python3 scripts/agent-feed-decision-contract/database_guard.py
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
export JWT_SECRET=public-local-feed-regression-key-only OMNISOLO_STANDALONE_MODE=false
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/agent-feed-decision-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/agent-feed-decision-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/agent-feed-decision-contract/verify_lock.py
python3 scripts/agent-feed-decision-contract/prepare.py
before=$(sha256sum scripts/agent-feed-decision-contract/source-manifest.json)
# The unchanged inherited repository lifecycle test reads this legacy variable.
# Give it a unique schema in the explicitly owned database, then require its
# durable effect below so its optional connection-failure return cannot pass.
lifecycle_schema="feed_lifecycle_$(python3 -c 'import uuid; print(uuid.uuid4().hex)')"
psql -X "$OHC_FEED_TEST_DATABASE_URL" -v ON_ERROR_STOP=1 -c "CREATE SCHEMA $lifecycle_schema" >/dev/null
cleanup_lifecycle() {
  psql -X "$OHC_FEED_TEST_DATABASE_URL" -v ON_ERROR_STOP=1 -c "SET client_min_messages=warning; DROP SCHEMA $lifecycle_schema CASCADE" >/dev/null
}
trap cleanup_lifecycle EXIT
export OMNISOLO_DATABASE_URL=$(python3 - "$lifecycle_schema" <<'PYURL'
import os,sys
from urllib.parse import urlencode,quote
print(os.environ['OHC_FEED_TEST_DATABASE_URL']+'?'+urlencode({'options':'-c search_path='+sys.argv[1]+' -c client_min_messages=warning'},quote_via=quote))
PYURL
)
psql -X "$OMNISOLO_DATABASE_URL" -v ON_ERROR_STOP=1 -f scripts/agent-feed-decision-contract/schema.sql >/dev/null
psql -X "$OMNISOLO_DATABASE_URL" -v ON_ERROR_STOP=1 -c "INSERT INTO tenants(id,name) VALUES('test-tenant-123','Owned lifecycle fixture')" >/dev/null
status=0
cargo test --locked --offline --manifest-path scripts/agent-feed-decision-contract/Cargo.toml "$@" -- --test-threads=1 || status=$?
if [[ $(psql -X "$OMNISOLO_DATABASE_URL" -v ON_ERROR_STOP=1 -Atc "SELECT count(*) FROM agent_feed_items WHERE tenant_id='test-tenant-123' AND event_source='test_source' AND lifecycle_state='APPROVED'") != 1 ]]; then
  echo 'Inherited repository::tests::test_agent_feed_repo_lifecycle did not execute its required owned PostgreSQL writes' >&2
  status=1
else
  echo 'Verified inherited repository::tests::test_agent_feed_repo_lifecycle executed its owned PostgreSQL writes'
fi
python3 scripts/agent-feed-decision-contract/prepare.py
test "$before" = "$(sha256sum scripts/agent-feed-decision-contract/source-manifest.json)"
exit "$status"

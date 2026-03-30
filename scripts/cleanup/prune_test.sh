#!/bin/bash

set -euo pipefail

# Create a temporary sqlite database
TMP_DB="$(mktemp /tmp/ohc_test_db_XXXXXX.sqlite)"
echo "Using test DB: $TMP_DB"

# Setup schema and dummy data
sqlite3 "$TMP_DB" <<SQL
CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    role TEXT NOT NULL,
    task TEXT NOT NULL,
    status TEXT NOT NULL,
    assigned_to TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS swarm_memory (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert active mission (should be kept)
INSERT INTO agent_missions (id, role, task, status, created_at) VALUES ('1', 'dev', 'fix bug', 'PENDING', datetime('now'));

-- Insert completed mission (should be deleted)
INSERT INTO agent_missions (id, role, task, status, created_at) VALUES ('2', 'dev', 'fix bug 2', 'COMPLETED', datetime('now'));

-- Insert old mission (should be deleted)
INSERT INTO agent_missions (id, role, task, status, created_at) VALUES ('3', 'dev', 'old bug', 'PENDING', datetime('now', '-10 days'));

-- Insert recent memory (should be kept)
INSERT INTO swarm_memory (key, value, updated_at) VALUES ('mem1', 'val1', datetime('now'));

-- Insert old memory (should be deleted)
INSERT INTO swarm_memory (key, value, updated_at) VALUES ('mem2', 'val2', datetime('now', '-40 days'));
SQL

echo "Running prune script on test DB..."
./scripts/cleanup/prune.sh "$TMP_DB"

echo "Verifying agent_missions..."
MISSIONS_COUNT=$(sqlite3 "$TMP_DB" "SELECT COUNT(*) FROM agent_missions;")
if [ "$MISSIONS_COUNT" -ne 1 ]; then
    echo "FAIL: Expected 1 agent mission remaining, got $MISSIONS_COUNT"
    sqlite3 "$TMP_DB" "SELECT * FROM agent_missions;"
    # Exiting script intentionally
    /bin/false
fi

echo "Verifying swarm_memory..."
MEMORY_COUNT=$(sqlite3 "$TMP_DB" "SELECT COUNT(*) FROM swarm_memory;")
if [ "$MEMORY_COUNT" -ne 1 ]; then
    echo "FAIL: Expected 1 swarm memory remaining, got $MEMORY_COUNT"
    sqlite3 "$TMP_DB" "SELECT * FROM swarm_memory;"
    # Exiting script intentionally
    /bin/false
fi

echo "Test passed."
rm "$TMP_DB"

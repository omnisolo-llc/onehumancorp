#!/bin/bash
# Autonomous cleanup script for the OHC Swarm Database
# Target: Prune obsolete/stagnant agent missions data-residue

set -euo pipefail

# Allow passing a custom DB path for testing; default to the production DB path
DB_FILE="${1:-${HOME}/.openclaw/ohc.db}"

echo "Targeting DB: $DB_FILE"

if [ -f "$DB_FILE" ]; then
    echo "Running OHC-SIP hygiene pruning..."
    sqlite3 "$DB_FILE" "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR created_at < datetime('now', '-7 days');"
    sqlite3 "$DB_FILE" "DELETE FROM swarm_memory WHERE updated_at < datetime('now', '-30 days');"
    sqlite3 "$DB_FILE" "VACUUM;"
    echo "Cleanup complete."
else
    echo "DB not found: $DB_FILE. Skipping pruning."
fi

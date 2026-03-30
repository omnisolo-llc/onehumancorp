#!/bin/bash
# Autonomous cleanup script for the OHC Swarm Database
# Target: Prune obsolete/stagnant agent missions data-residue
# And clean up internal tools/scratch files and generated protobuf files.

DB_FILE="${HOME}/.openclaw/ohc.db"
if [ -f "$DB_FILE" ]; then
    echo "Running OHC-SIP hygiene pruning..."
    sqlite3 "$DB_FILE" "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR created_at < datetime('now', '-7 days');"
    sqlite3 "$DB_FILE" "DELETE FROM swarm_memory WHERE updated_at < datetime('now', '-30 days');"
    sqlite3 "$DB_FILE" "VACUUM;"
    echo "Cleanup complete."
else
    echo "DB not found: $DB_FILE. Skipping."
fi

# Search and destroy generated protobuf files
find . -type f \( -name "*.pb.go" -o -name "*.pb.ts" -o -name "*_pb2.py" \) -exec rm -f {} +
echo "Removed generated protobuf files."

# Cleanup git untracked branches if necessary
git remote prune origin || true

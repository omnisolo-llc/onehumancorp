#!/bin/bash
set -e

# Janitor cleanup script to remove obsolete data-residue and prune superseded architectural blueprints

echo "Starting cleanup..."

# Remove temporary data residue files (Zero Junk policy)
find . -type f -name "*.log" -delete
find . -type f -name "*.patch" -delete
find . -type f -name "triage_report.html" -delete
find . -type f -name "*.diff" -delete
find . -type f -name "*.pb.go" -delete
find . -type f -name "*.pb.ts" -delete
find . -type f -name "*_pb2.py" -delete

# Prune superseded blueprints from swarm_memory
# This safely deletes any architectural_blueprint_v* that is not the latest version (v3)
DB_PATH="$HOME/.openclaw/ohc.db"
if [ -f "$DB_PATH" ]; then
    sqlite3 "$DB_PATH" "DELETE FROM swarm_memory WHERE key LIKE 'architectural_blueprint_v%' AND key != 'architectural_blueprint_v3';"
    echo "Pruned obsolete blueprints from swarm_memory."
else
    echo "DB not found at $DB_PATH, skipping database pruning."
fi

echo "Cleanup completed successfully."

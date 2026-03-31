#!/bin/bash
set -e

echo "Starting cleanup process..."

# 1. Prune obsolete generated protobuf files
find . -type f \( -name '*.pb.go' -o -name '*.pb.ts' -o -name '*_pb2.py' \) -print0 | xargs -0 rm -f

# 2. Remove temporary python/bash scripts
# We'll safely limit to just the root dir and specifically not `venv` or known scripts.
find . -maxdepth 1 -name "*.py" ! -path "*/venv/*" ! -path "*/.venv/*" -delete
find . -maxdepth 1 -name "*.sh" ! -path "*/scripts/*" ! -path "*/deploy/*" ! -path "*/.github/*" ! -path "*/.git/*" -delete

# 3. Clean up .agent-task/ (obsolete missions, memory, status)
# Since yaml module is not available, we use awk/grep directly.

# Clean agent_missions table in DB where status is DONE
sqlite3 .agent-task/swarm.db "DELETE FROM agent_missions WHERE status = 'DONE';" || true

# Clean swarm_memory table in DB
sqlite3 .agent-task/swarm.db "DELETE FROM swarm_memory WHERE type = 'QUEUE_EMPTY';" || true
sqlite3 .agent-task/swarm.db "DELETE FROM swarm_memory WHERE content LIKE '%Queue is empty%';" || true
sqlite3 .agent-task/swarm.db "DELETE FROM swarm_memory WHERE content LIKE '%Queue was empty%';" || true

# Remove files with 'DONE' or 'COMPLETED' or 'proposed' status in .agent-task/missions/
for file in .agent-task/missions/*.yml; do
    if [ -f "$file" ]; then
        if grep -qE "^status: *[\"']?(DONE|COMPLETED|proposed)[\"']?" "$file"; then
            rm -f "$file"
        elif grep -qE "^description:.*SemVer calculation" "$file"; then
            rm -f "$file"
        fi
    fi
done

# Remove files with 'DONE', 'COMPLETED', 'offline' or QUEUE_EMPTY memory_type in .agent-task/status/
for file in .agent-task/status/*.yml; do
    if [ -f "$file" ]; then
        if grep -qE "^status: *[\"']?(DONE|COMPLETED|offline|QUEUE_EMPTY)[\"']?" "$file" || grep -qE "^memory_type: *[\"']?QUEUE_EMPTY[\"']?" "$file" || grep -q "Queue was empty, exiting successfully." "$file" || grep -q "queue_status: *[\"']?empty[\"']?" "$file"; then
            rm -f "$file"
        elif grep -qE "^description:.*Executing Final Mile tasks" "$file"; then
            rm -f "$file"
        fi
    fi
done

# Remove memory files that mention 'Queue is empty' or 'Queue was empty'
for file in .agent-task/memory/*.yml; do
    if [ -f "$file" ]; then
        if grep -qE "(Queue is empty|Queue was empty)" "$file"; then
            rm -f "$file"
        fi
    fi
done

echo 'Cleanup complete'

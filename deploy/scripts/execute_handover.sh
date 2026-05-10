#!/bin/bash
set -e

if [ -z "$DATABASE_URL" ]; then
  echo "Error: DATABASE_URL environment variable is not set."
  exit 1
fi

echo "Executing Mission Handover Protocol..."
psql "$DATABASE_URL" -c "UPDATE agent_missions SET status = 'blocked', mission_log = COALESCE(mission_log, '') || chr(10) || 'Mission blocked: lack of specifications.' WHERE status = 'PENDING';"
echo "Handover complete."

#!/usr/bin/env bash
curl -X POST http://localhost:8080/api/missions/sync \
-H "Content-Type: application/json" \
-H "X-OHC-Conflict-Resolution: force-local" \
-d '{"role": "SOFTWARE_ENGINEER", "task": {"id": "123", "type": "TASK", "content": "Write some code"}}'

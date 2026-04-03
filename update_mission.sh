#!/bin/bash
MISSION_FILE=".agent-task/missions/2026-04-03T17-41-44Z.md"
sed -i 's/^status: PENDING/status: IN_PROGRESS\nagent: jules/' "$MISSION_FILE"

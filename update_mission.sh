#!/bin/bash
MISSION_FILE=".agent-task/missions/2026-04-05T09-49-05Z.md"
sed -i 's/status: IN_PROGRESS/status: DONE/' "$MISSION_FILE"

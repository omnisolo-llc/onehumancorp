#!/bin/bash
file=".agent-task/missions/2026-04-05T22-37-53Z_kairos_teammate_mesh_apis.md"
sed -i 's/status: PENDING/status: DONE\nagent: Jules/g' "$file"

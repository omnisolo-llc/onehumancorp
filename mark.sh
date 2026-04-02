cat << 'INNER_EOF' > tmp_mission.md
---
status: IN_PROGRESS
agent: Jules
---
INNER_EOF
cat .agent-task/missions/2026-04-02T09-42-18Z.md >> tmp_mission.md
mv tmp_mission.md .agent-task/missions/2026-04-02T09-42-18Z.md

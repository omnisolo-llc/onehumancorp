cat << 'INNER_EOF' > tmp_mission.md
---
status: DONE
agent: Jules
---
INNER_EOF
tail -n +4 .agent-task/missions/2026-04-02T09-42-18Z.md >> tmp_mission.md
mv tmp_mission.md .agent-task/missions/2026-04-02T09-42-18Z.md

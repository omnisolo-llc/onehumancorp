import re
with open('.agent-task/missions/2026-04-04T11-01-02Z_kairos_orchestration.md', 'r') as f:
    content = f.read()

content = content.replace('status: IN_PROGRESS', 'status: DONE')
with open('.agent-task/missions/2026-04-04T11-01-02Z_kairos_orchestration.md', 'w') as f:
    f.write(content)

import re

with open(".agent-task/missions/2026-04-05T12-40-00Z_kairos_orchestrator.md", "r") as f:
    content = f.read()

content = re.sub(r'status: PENDING', 'status: DONE', content)
content = re.sub(r'agent: Implementer', 'agent: Jules', content)

with open(".agent-task/missions/2026-04-05T12-40-00Z_kairos_orchestrator.md", "w") as f:
    f.write(content)

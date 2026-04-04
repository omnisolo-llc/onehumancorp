import re

with open(".agent-task/missions/2026-04-04T12-00-00Z_kairos_shared_task_list.md", "r") as f:
    content = f.read()

content = re.sub(r"status: IN_PROGRESS\nagent: Implementer\nagent: Implementer", "status: IN_PROGRESS\nagent: Implementer", content)

with open(".agent-task/missions/2026-04-04T12-00-00Z_kairos_shared_task_list.md", "w") as f:
    f.write(content)

import re
import io

with open('.agent-task/missions/2026-04-06T00-40-58Z_teammate_mesh_apis.md', 'r') as f:
    content = f.read()

content = re.sub(r'status:\s*"IN_PROGRESS"', 'status: "DONE"', content)

with open('.agent-task/missions/2026-04-06T00-40-58Z_teammate_mesh_apis.md', 'w') as f:
    f.write(content)

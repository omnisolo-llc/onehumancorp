import re

with open(".agent-task/missions/2026-04-12T22-25-19Z_kairos_teammate_mesh_dashboard.md", "r") as f:
    content = f.read()

content = re.sub(r'status: PENDING', 'status: DONE\nagent: Echo', content)

with open(".agent-task/missions/2026-04-12T22-25-19Z_kairos_teammate_mesh_dashboard.md", "w") as f:
    f.write(content)

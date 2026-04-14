import re

with open('.agent-task/missions/2026-04-14T15-38-05Z_kairos_mesh.md', 'r') as f:
    content = f.read()

new_content = content.replace('---\n', '---\nstatus: DONE\nagent: Implementer\n', 1)

with open('.agent-task/missions/2026-04-14T15-38-05Z_kairos_mesh.md', 'w') as f:
    f.write(new_content)

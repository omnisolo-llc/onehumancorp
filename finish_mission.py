import sys

with open('.agent-task/missions/2026-04-03T17-41-44Z.md', 'r') as f:
    content = f.read()

content = content.replace('status: IN_PROGRESS', 'status: DONE')

with open('.agent-task/missions/2026-04-03T17-41-44Z.md', 'w') as f:
    f.write(content)

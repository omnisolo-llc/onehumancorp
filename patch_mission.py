import re

with open(".agent-task/missions/2026-04-03T14-24-10Z.md", "r") as f:
    content = f.read()

content = re.sub(r'status: BLOCKED', 'status: IN_PROGRESS', content)
content = re.sub(r'agent: Scribe', 'agent: Jules', content)

with open(".agent-task/missions/2026-04-03T14-24-10Z.md", "w") as f:
    f.write(content)

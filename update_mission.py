import re

with open(".agent-task/missions/2026-04-05T09-49-05Z.md", "r") as f:
    content = f.read()

if "status: " not in content:
    content = "status: IN_PROGRESS\nagent: jules\n" + content

with open(".agent-task/missions/2026-04-05T09-49-05Z.md", "w") as f:
    f.write(content)

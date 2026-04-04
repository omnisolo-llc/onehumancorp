import re

with open(".agent-task/missions/1775220000_nova_proactive_growth.md", "r") as f:
    content = f.read()

content = content.replace("status: DONE", "status: DONE\nagent: Nova")

with open(".agent-task/missions/1775220000_nova_proactive_growth.md", "w") as f:
    f.write(content)

with open('.agent-task/missions/2026-04-16T05-24-39Z.md', 'r') as f:
    content = f.read()

content = content.replace("status: PENDING", "status: DONE")
content = content.replace("priority:", "agent: implementer\npriority:")

with open('.agent-task/missions/2026-04-16T05-24-39Z.md', 'w') as f:
    f.write(content)

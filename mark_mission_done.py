with open(".agent-task/missions/2026-04-06T02-49-45Z.md", "r") as f:
    content = f.read()
content = "---\nstatus: DONE\nagent: Scribe\n---\n" + content
with open(".agent-task/missions/2026-04-06T02-49-45Z.md", "w") as f:
    f.write(content)

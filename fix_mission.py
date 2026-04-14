import re
with open('.agent-task/missions/2026-04-14T14-49-50Z.md', 'r') as f:
    c = f.read()

c = c.replace('<<<<<<< HEAD\n---\nstatus: DONE\nagent: Implementer\n---\n=======\n>>>>>>> origin/main\n', '---\nstatus: DONE\nagent: Implementer\n---\n')

with open('.agent-task/missions/2026-04-14T14-49-50Z.md', 'w') as f:
    f.write(c)

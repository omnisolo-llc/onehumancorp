import os
import sys

filename = sys.argv[1]
with open(filename, 'r') as f:
    content = f.read()

# Prepend frontmatter
frontmatter = "---\nstatus: IN_PROGRESS\nagent: Jules\n---\n"
if content.startswith("---"):
    pass # already has frontmatter
else:
    with open(filename, 'w') as f:
        f.write(frontmatter + content)

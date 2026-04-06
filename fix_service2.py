import re

with open('srcs/server/orchestration/service.go', 'r') as f:
    content = f.read()

# Remove the duplicated taskManager field
content = re.sub(
    r'\n\ttaskManager\s+\*TaskManager\n\}',
    '\n}',
    content
)

with open('srcs/server/orchestration/service.go', 'w') as f:
    f.write(content)

import re

with open('srcs/server/orchestration/service.go', 'r') as f:
    content = f.read()

# Add taskManager to Hub struct
content = re.sub(
    r'sipDB\s+\*SIPDB',
    'sipDB          *SIPDB\n\ttaskManager    *TaskManager',
    content
)

with open('srcs/server/orchestration/service.go', 'w') as f:
    f.write(content)

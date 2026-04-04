import re

# Fix imported and not used in synchronizer.go
with open('srcs/server/orchestration/synchronizer.go', 'r') as f:
    content = f.read()
content = re.sub(r'^\s*"fmt"\n', '', content, flags=re.MULTILINE)
with open('srcs/server/orchestration/synchronizer.go', 'w') as f:
    f.write(content)

# Fix imported and not used in synchronizer_test.go
with open('srcs/server/orchestration/synchronizer_test.go', 'r') as f:
    content = f.read()
content = re.sub(r'^\s*"time"\n', '', content, flags=re.MULTILINE)
with open('srcs/server/orchestration/synchronizer_test.go', 'w') as f:
    f.write(content)

# Fix imported and not used in tasks_test.go
with open('srcs/server/orchestration/tasks_test.go', 'r') as f:
    content = f.read()
content = re.sub(r'^\s*"time"\n', '', content, flags=re.MULTILINE)
with open('srcs/server/orchestration/tasks_test.go', 'w') as f:
    f.write(content)

import sys

with open('srcs/server/orchestration/tasks_store_test.go', 'r') as f:
    content = f.read()

# I thought I replaced mockRow but I see it's still complaining about mockRow. Let's do it again properly.
import re
content = re.sub(r'\bmockRow\b', 'storeMockRow', content)
content = re.sub(r'\bmockTx\b', 'storeMockTx', content)
content = re.sub(r'\bmockDB\b', 'storeMockDB', content)

with open('srcs/server/orchestration/tasks_store_test.go', 'w') as f:
    f.write(content)

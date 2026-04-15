import sys

with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    content = f.read()

import re
# If tasks_db.go is in the list twice, remove it
content = re.sub(r'("tasks_db.go",\s*)+', '"tasks_db.go",\n        ', content)
content = re.sub(r'("tasks_store.go",\s*)+', '"tasks_store.go",\n        ', content)
content = re.sub(r'("tasks_store_test.go",\s*)+', '"tasks_store_test.go",\n        ', content)

# But wait, did I put it twice or was it already there?
# Let's just remove them and put them exactly once at the top of srcs to be safe, or just check

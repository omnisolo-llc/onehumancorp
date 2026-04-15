import sys

with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    content = f.read()

# Replace duplicates
content = content.replace('        "tasks_db.go",\n', '', 1)
content = content.replace('        "tasks_store.go",\n', '', 1)
content = content.replace('        "tasks_store_test.go",\n', '', 1)

with open('srcs/server/orchestration/BUILD.bazel', 'w') as f:
    f.write(content)

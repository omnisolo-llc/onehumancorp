import sys

with open('srcs/server/orchestration/tasks_store_test.go', 'r') as f:
    content = f.read()

# Replace db.CommandTag
content = content.replace('(db.CommandTag, error)', '(int64, error)')
content = content.replace('return nil, nil', 'return 0, nil')

with open('srcs/server/orchestration/tasks_store_test.go', 'w') as f:
    f.write(content)

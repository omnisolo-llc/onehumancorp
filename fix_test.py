with open('srcs/server/orchestration/tasks_db_test.go', 'r') as f:
    content = f.read()

content = content.replace(
    '\'["task-1"]\')"',
    '\'["task-1"]\')`'
).replace(
    '"INSERT INTO shared_tasks_v4',
    '`INSERT INTO shared_tasks_v4'
).replace(
    '\'["task-2"]\')"',
    '\'["task-2"]\')`'
)

with open('srcs/server/orchestration/tasks_db_test.go', 'w') as f:
    f.write(content)

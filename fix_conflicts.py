with open('srcs/server/db/BUILD.bazel', 'r') as f:
    c = f.read()
c = c.replace('<<<<<<< HEAD\n        "migrations/20260412_shared_tasks_decomposition.sql",\n=======\n>>>>>>> origin/main\n', '        "migrations/20260412_shared_tasks_decomposition.sql",\n')
with open('srcs/server/db/BUILD.bazel', 'w') as f:
    f.write(c)

with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    c = f.read()

c = c.replace('<<<<<<< HEAD\n        "models.go",\n=======\n>>>>>>> origin/main\n', '        "models.go",\n')
c = c.replace('<<<<<<< HEAD\n        "//srcs/server/auth",\n=======\n>>>>>>> origin/main\n', '        "//srcs/server/auth",\n')

with open('srcs/server/orchestration/BUILD.bazel', 'w') as f:
    f.write(c)

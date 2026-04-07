import re

with open("srcs/server/orchestration/autodream_test.go", "r") as f:
    content = f.read()

# Instead of passing the pool directly, maybe we just don't need to do any real DB inserts that conflict with the transaction inside pruneStaleSessions
content = content.replace("worker.pruneStaleSessions(context.Background())", "worker.pruneStaleSessions(ctx)")

with open("srcs/server/orchestration/autodream_test.go", "w") as f:
    f.write(content)

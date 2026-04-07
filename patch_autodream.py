import re

# In TestAutoDreamPruneSessions, the global lock or something is freezing. Let's look at cached_minimax_client.go too
# Ah, cached_minimax_client uses the DB inside Reason, but maybe we passed a pool to the mock that is waiting for the tx to commit?
# Let's fix the test to not run into db lock issues.
with open("srcs/server/orchestration/autodream_test.go", "r") as f:
    content = f.read()

# Instead of passing the pool directly, maybe we just don't need to do any real DB inserts that conflict with the transaction inside pruneStaleSessions
content = content.replace("worker.pruneStaleSessions(ctx)", "worker.pruneStaleSessions(context.Background())")

with open("srcs/server/orchestration/autodream_test.go", "w") as f:
    f.write(content)

import re
with open('srcs/server/orchestration/test_provider_test.go', 'r') as f:
    content = f.read()

# Instead of creating my own TestProvider which is missing methods,
# wait, wait!
# Why not just revert EVERYTHING in `srcs/server/db` and JUST copy `test_provider.go` content into `test_provider_test.go` and put it in `orchestration`?
# Let's look at `srcs/server/db/test_provider.go`

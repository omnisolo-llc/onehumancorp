import re

with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()

# Fix the syntax error in test
content = content.replace("tests := [)struct {", "tests := []struct {")

with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)

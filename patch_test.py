import re

with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()

# Fix the msgs[0) to msgs[0]
content = content.replace("msgs[0)", "msgs[0]")
content = content.replace("hub.agents", "hub.Agents()")

with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)

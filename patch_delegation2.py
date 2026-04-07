import re

with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()

# Fix the syntax error in test
content = content.replace("hub.FireAgent(\\\"sender-fail\\\")", "hub.FireAgent(\"sender-fail\")")
content = content.replace("for _, agent := range hub.Agents() {\\n\\tid := agent.ID", "for _, agent := range hub.Agents() {\n\t\tid := agent.ID")

with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)

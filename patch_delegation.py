import re

with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()

# Fix the map iteration over hub.Agents() which now returns a slice
content = content.replace("for id := range hub.Agents() {", "for _, agent := range hub.Agents() {\\n\\tid := agent.ID")
content = content.replace("delete(hub.Agents(), \\\"sender-fail\\\")", "hub.FireAgent(\\\"sender-fail\\\")")

with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)

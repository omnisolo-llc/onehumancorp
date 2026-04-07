import re

# Fix delegation.go
with open("srcs/server/orchestration/delegation.go", "r") as f:
    content = f.read()

content = content.replace("s.hub.agents[subAgent.ID]", "agent")
content = content.replace(
"""	if _, exists := agent; !exists {
		agent = subAgent
	}""",
"""	if _, exists := s.hub.Agent(subAgent.ID); !exists {
		s.hub.RegisterAgent(subAgent)
	}""")

with open("srcs/server/orchestration/delegation.go", "w") as f:
    f.write(content)


# Fix delegation_test.go
with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()

content = content.replace("delete(hub.Agents(), \\\"sender-fail\\\")", "hub.FireAgent(\\\"sender-fail\\\")")
content = content.replace("delete(hub.Agents(), \"sender-fail\")", "hub.FireAgent(\"sender-fail\")")

with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)

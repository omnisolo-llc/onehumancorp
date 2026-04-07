import re

# Fix delegation.go
with open("srcs/server/orchestration/delegation.go", "r") as f:
    content = f.read()

content = content.replace("s.hub.agents[req.GetAgentId()] = s.hub.agents[req.GetAgentId()]", "/* skip directly modifying hub.agents */")
content = content.replace("s.hub.agents[req.GetAgentId()]", "agent")
content = content.replace("_, ok := s.hub.agents[req.GetAgentId()]", "agent, ok := s.hub.Agent(req.GetAgentId())")
content = content.replace("agent, ok := s.hub.agents[req.GetAgentId()]", "agent, ok := s.hub.Agent(req.GetAgentId())")

with open("srcs/server/orchestration/delegation.go", "w") as f:
    f.write(content)

# Fix delegation_test.go
with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()

content = content.replace("hub.agents[", "hub.Agent(")
content = content.replace("]", ")")
content = content.replace("hub.inbox[", "hub.Inbox(")
content = content.replace("hub.inbox", "hub.Inbox")

with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)

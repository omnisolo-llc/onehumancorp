import re

with open('srcs/server/tools/hybridfsmcp/mcp_test.go', 'r') as f:
    content = f.read()

content = content.replace("mcp := NewHybridFSMCP(provider)", "mcp := NewHybridFSMCP(provider, nil)")

with open('srcs/server/tools/hybridfsmcp/mcp_test.go', 'w') as f:
    f.write(content)

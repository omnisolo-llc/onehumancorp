import re

with open('srcs/server/tools/hybridfsmcp/mcp.go', 'r') as f:
    content = f.read()

content = content.replace("func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP", "func NewHybridFSMCP(provider FileSystemProvider, escalator interface{}) *HybridFSMCP")

with open('srcs/server/tools/hybridfsmcp/mcp.go', 'w') as f:
    f.write(content)

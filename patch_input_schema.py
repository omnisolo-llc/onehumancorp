with open("srcs/server/tools/hybridfsmcp/mcp.go", "r") as f:
    c = f.read()

c = c.replace(
    'InputSchema string `json:"inputSchema"`',
    'InputSchema map[string]interface{} `json:"inputSchema"`'
)
c = c.replace(
    '`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`',
    'map[string]interface{}{\n\t\t\t\t"type": "object",\n\t\t\t\t"properties": map[string]interface{}{\n\t\t\t\t\t"path": map[string]interface{}{"type": "string"},\n\t\t\t\t},\n\t\t\t\t"required": []string{"path"},\n\t\t\t}'
)
c = c.replace(
    '`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`',
    'map[string]interface{}{\n\t\t\t\t"type": "object",\n\t\t\t\t"properties": map[string]interface{}{\n\t\t\t\t\t"path": map[string]interface{}{"type": "string"},\n\t\t\t\t\t"content": map[string]interface{}{"type": "string"},\n\t\t\t\t},\n\t\t\t\t"required": []string{"path", "content"},\n\t\t\t}'
)
c = c.replace(
    '`{"type": "object", "properties": {"path": {"type": "string"}, "query": {"type": "string"}}, "required": ["path", "query"]}`',
    'map[string]interface{}{\n\t\t\t\t"type": "object",\n\t\t\t\t"properties": map[string]interface{}{\n\t\t\t\t\t"path": map[string]interface{}{"type": "string"},\n\t\t\t\t\t"query": map[string]interface{}{"type": "string"},\n\t\t\t\t},\n\t\t\t\t"required": []string{"path", "query"},\n\t\t\t}'
)

with open("srcs/server/tools/hybridfsmcp/mcp.go", "w") as f:
    f.write(c)

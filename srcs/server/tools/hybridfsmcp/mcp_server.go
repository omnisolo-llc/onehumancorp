package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

// Standard MCP tool definitions
func GetMCPTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "read_file",
			"description": "Read contents of a file",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "Path to the file relative to the base directory",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			"name":        "write_file",
			"description": "Write contents to a file",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "Path to the file relative to the base directory",
					},
					"data": map[string]interface{}{
						"type":        "string",
						"description": "Content to write to the file",
					},
				},
				"required": []string{"path", "data"},
			},
		},
		{
			"name":        "list_directory",
			"description": "List files in a directory",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "Path to the directory relative to the base directory",
					},
				},
				"required": []string{"path"},
			},
		},
	}
}

// CallTool implements the standard MCP call logic
func (s *Server) CallTool(ctx context.Context, name string, argsRaw json.RawMessage) (interface{}, error) {
	switch name {
	case "read_file":
		return s.ReadFile(ctx, argsRaw)
	case "write_file":
		return s.WriteFile(ctx, argsRaw)
	case "list_directory":
		return s.ListDir(ctx, argsRaw)
	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

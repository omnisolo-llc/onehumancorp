package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

type Tool struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	InputSchema interface{} `json:"inputSchema"`
}

func NewHybridFSMCP() *HybridFSMCP {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &HybridFSMCP{provider: NewCloudFSProvider()}
	}
	return &HybridFSMCP{provider: NewLocalFSProvider()}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the file system.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file in the file system.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(path, claims)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"content": string(data),
		}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		// Avoid base64 decoding heuristics. Treat content strictly as a normal string.
		err := m.provider.WriteFile(path, []byte(content), claims)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		path := ""
		if p, ok := arguments["path"].(string); ok {
			path = p
		}

		infos, err := m.provider.ListDir(path, claims)
		if err != nil {
			return nil, err
		}

		var results []map[string]interface{}
		for _, info := range infos {
			results = append(results, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
			})
		}
		return map[string]interface{}{
			"status":  "success",
			"results": results,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

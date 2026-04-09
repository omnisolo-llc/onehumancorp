package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	InputSchema interface{} `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for filesystem access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on mode.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	readSchema := map[string]interface{}{
		"type": "object",
		"properties": map[string]interface{}{
			"path": map[string]interface{}{"type": "string"},
		},
		"required": []string{"path"},
	}

	writeSchema := map[string]interface{}{
		"type": "object",
		"properties": map[string]interface{}{
			"path":    map[string]interface{}{"type": "string"},
			"content": map[string]interface{}{"type": "string"},
		},
		"required": []string{"path", "content"},
	}

	listSchema := map[string]interface{}{
		"type": "object",
		"properties": map[string]interface{}{
			"path": map[string]interface{}{"type": "string"},
		},
		"required": []string{"path"},
	}

	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: readSchema,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: writeSchema,
		},
		{
			Name:        "list_directory",
			Description: "Lists contents of a directory.",
			InputSchema: listSchema,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
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

		err := m.provider.WriteFile(ctx, claims, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		infos, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}

		var results []map[string]interface{}
		for _, info := range infos {
			results = append(results, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
				"time":  info.ModTime().Format(time.RFC3339),
			})
		}

		return map[string]interface{}{
			"status": "success",
			"items":  results,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

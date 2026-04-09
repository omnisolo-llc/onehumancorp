package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type HybridFSMCP struct {
	provider mcp.FileSystemProvider
}

func NewHybridFSMCP(isStandalone bool, workspaceDir string) *HybridFSMCP {
	var provider mcp.FileSystemProvider
	if isStandalone {
		provider = mcp.NewLocalFSProvider(workspaceDir)
	} else {
		provider = mcp.NewCloudFSProvider(workspaceDir)
	}
	return &HybridFSMCP{provider: provider}
}

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes a file to the file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists a directory in the file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
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
		err := m.provider.WriteFile(ctx, path, []byte(content))
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
		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		var results []map[string]interface{}
		for _, e := range entries {
			info, err := e.Info()
			if err != nil {
				continue
			}
			results = append(results, map[string]interface{}{
				"name":          e.Name(),
				"is_dir":        e.IsDir(),
				"size":          info.Size(),
				"last_modified": info.ModTime().Format(time.RFC3339),
			})
		}
		return map[string]interface{}{
			"status":  "success",
			"entries": results,
		}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
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
			Description: "Reads the content of a file at the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file at the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in the given path.",
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
		return m.readFile(ctx, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		content := []byte(contentStr)

		return m.writeFile(ctx, path, content)
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, path)

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status":  "success",
		"content": string(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, content []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, content)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status": "success",
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	infos, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, info := range infos {
		results = append(results, map[string]interface{}{
			"name":     info.Name,
			"size":     info.Size,
			"is_dir":   info.IsDir,
			"mod_time": info.ModTime,
		})
	}

	return map[string]interface{}{
		"status": "success",
		"files":  results,
	}, nil
}

// Factory logic
func NewFactoryProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		pvDir := os.Getenv("OHC_TENANT_PV_DIR")
		return NewCloudFSProvider(pvDir)
	}
	workspaceDir := os.Getenv("OHC_WORKSPACE_DIR")
	return NewLocalFSProvider(workspaceDir)
}

func NewHybridFSProxy() *HybridFSMCP {
	return NewHybridFSMCP(NewFactoryProvider())
}

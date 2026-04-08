package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for hybrid file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance, instantiating the correct
// provider based on the OHC_STANDALONE environment variable.
func NewHybridFSMCP(baseDir string) (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_STANDALONE") == "true" {
		provider, err = NewLocalFSProvider(baseDir)
	} else {
		provider, err = NewCloudFSProvider(baseDir)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to initialize FS provider: %w", err)
	}

	return &HybridFSMCP{
		provider: provider,
	}, nil
}

// ListTools returns the list of available tools.
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
			Description: "Lists files and directories under a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes the specified tool with the provided input.
func (m *HybridFSMCP) CallTool(ctx context.Context, name string, input []byte) ([]byte, error) {
	switch name {
	case "read_file":
		var req struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(input, &req); err != nil {
			return nil, fmt.Errorf("invalid input for read_file: %w", err)
		}
		data, err := m.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		return json.Marshal(map[string]string{"content": string(data)})

	case "write_file":
		var req struct {
			Path    string `json:"path"`
			Content string `json:"content"`
		}
		if err := json.Unmarshal(input, &req); err != nil {
			return nil, fmt.Errorf("invalid input for write_file: %w", err)
		}
		if err := m.provider.WriteFile(ctx, req.Path, []byte(req.Content)); err != nil {
			return nil, err
		}
		return json.Marshal(map[string]string{"status": "success"})

	case "list_directory":
		var req struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(input, &req); err != nil {
			return nil, fmt.Errorf("invalid input for list_directory: %w", err)
		}
		entries, err := m.provider.ListDir(ctx, req.Path)
		if err != nil {
			return nil, err
		}

		var result []map[string]interface{}
		for _, e := range entries {
			result = append(result, map[string]interface{}{
				"name":  e.Name(),
				"isDir": e.IsDir(),
			})
		}
		return json.Marshal(map[string]interface{}{"entries": result})

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

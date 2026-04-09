package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"
)

// HybridFSMCP implements the MCP interface for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
	isLocal  bool
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on the mode.
func NewHybridFSMCP(isLocal bool, baseDir string) (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	if isLocal {
		provider, err = NewLocalFSProvider(baseDir)
	} else {
		provider, err = NewCloudFSProvider(baseDir)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSMCP{
		provider: provider,
		isLocal:  isLocal,
	}, nil
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available file system tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file at the specified path. Returns content encoded in base64 to handle binary data.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file at the specified path. Content should be provided as a base64 encoded string.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a file system tool by name.
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
		content, err := base64.StdEncoding.DecodeString(contentStr)
		if err != nil {
			return nil, fmt.Errorf("failed to decode base64 content: %w", err)
		}
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

func (m *HybridFSMCP) getModeString() string {
	if m.isLocal {
		return "standalone"
	}
	return "cloud"
}

func (m *HybridFSMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getModeString(),
		"content": base64.StdEncoding.EncodeToString(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, content []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, content)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   m.getModeString(),
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getModeString(),
		"entries": entries,
	}, nil
}

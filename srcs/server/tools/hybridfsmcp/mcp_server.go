package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
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

// NewHybridFSMCP creates a new HybridFSMCP instance based on environment.
func NewHybridFSMCP() *HybridFSMCP {
	rootPath := os.Getenv("OHC_FS_ROOT")
	if rootPath == "" {
		rootPath = os.TempDir()
	}

	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	var provider FileSystemProvider
	if isMultiTenant {
		provider = NewCloudFSProvider(rootPath)
	} else {
		provider = NewLocalFSProvider(rootPath)
	}

	return &HybridFSMCP{
		provider: provider,
	}
}

// NewHybridFSMCPWithProvider creates a new HybridFSMCP with a specific provider (useful for testing).
func NewHybridFSMCPWithProvider(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file. Content should be base64 encoded.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if !m.provider.IsLocal() {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil {
			return nil, errors.New("unauthorized: missing claims")
		}
	}

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
		return m.writeFile(ctx, path, contentStr)
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
	content, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"content": base64.StdEncoding.EncodeToString(content),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, contentStr string) (interface{}, error) {
	content, err := base64.StdEncoding.DecodeString(contentStr)
	if err != nil {
		return nil, fmt.Errorf("failed to decode content: %w", err)
	}

	err = m.provider.WriteFile(ctx, path, content)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, e := range entries {
		results = append(results, map[string]interface{}{
			"name":  e.Name(),
			"size":  e.Size(),
			"isDir": e.IsDir(),
		})
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"entries": results,
	}, nil
}

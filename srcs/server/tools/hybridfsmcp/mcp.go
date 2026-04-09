package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HybridFSMCP implements the MCP interface for hybrid file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on the environment mode.
func NewHybridFSMCP(rootDir string) (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_STANDALONE") == "true" {
		provider, err = NewLocalFSProvider(rootDir)
	} else {
		provider, err = NewCloudFSProvider(rootDir)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSMCP{provider: provider}, nil
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file at the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file at the given path. Content must be base64 encoded.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content_base64": {"type": "string"}}, "required": ["path", "content_base64"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists the entries in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
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
		content, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		// Return base64 encoded content to be safe with binary files
		return map[string]interface{}{"content_base64": base64.StdEncoding.EncodeToString(content)}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		contentBase64, ok := arguments["content_base64"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content_base64' argument")
		}

		content, err := base64.StdEncoding.DecodeString(contentBase64)
		if err != nil {
			return nil, fmt.Errorf("invalid base64 content: %w", err)
		}

		err = m.provider.WriteFile(ctx, claims, path, content)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

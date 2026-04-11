package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCPServer creates a new HybridFSMCP instance.
func NewHybridFSMCPServer(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "The path of the file to read."}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "The path of the file to write to."}, "content": {"type": "string", "description": "The contents to write."}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "List the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "The directory path to list."}}, "required": ["path"]}`),
		},
		{
			Name:        "search_files",
			Description: "Search for files matching a pattern in a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string", "description": "The directory path to search in."}, "pattern": {"type": "string", "description": "The string pattern to search for in file names."}}, "required": ["path", "pattern"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	// We allow nil claims if the provider allows it (e.g. LocalFSProvider).
	// CloudFSProvider will check and fail if claims are missing.

	pathRaw, ok := arguments["path"]
	if !ok {
		return nil, errors.New("missing required argument 'path'")
	}
	path, ok := pathRaw.(string)
	if !ok {
		return nil, errors.New("argument 'path' must be a string")
	}

	switch toolName {
	case "read_file":
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"content": string(data),
		}, nil

	case "write_file":
		contentRaw, ok := arguments["content"]
		if !ok {
			return nil, errors.New("missing required argument 'content'")
		}
		content, ok := contentRaw.(string)
		if !ok {
			return nil, errors.New("argument 'content' must be a string")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"entries": entries,
		}, nil

	case "search_files":
		patternRaw, ok := arguments["pattern"]
		if !ok {
			return nil, errors.New("missing required argument 'pattern'")
		}
		pattern, ok := patternRaw.(string)
		if !ok {
			return nil, errors.New("argument 'pattern' must be a string")
		}
		matches, err := m.provider.SearchFiles(ctx, claims, path, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"matches": matches,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

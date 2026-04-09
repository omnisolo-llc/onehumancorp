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

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(baseDir string) (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	// According to memory context, OHC_STANDALONE="false" indicates Cloud mode
	if os.Getenv("OHC_STANDALONE") == "false" {
		provider, err = NewCloudFSProvider(baseDir)
	} else {
		provider, err = NewLocalFSProvider(baseDir)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSMCP{provider: provider}, nil
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
			Description: "Writes content to a file at the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}, "encoding": {"type": "string", "enum": ["utf-8", "base64"]}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern.",
			InputSchema: `{"type": "object", "properties": {"pattern": {"type": "string"}}, "required": ["pattern"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && os.Getenv("OHC_STANDALONE") == "false" {
		return nil, ErrUnauthorized
	}

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

		encoding, ok := arguments["encoding"].(string)
		if !ok {
			encoding = "utf-8"
		}

		var data []byte
		var err error
		if encoding == "base64" {
			data, err = base64.StdEncoding.DecodeString(content)
			if err != nil {
				return nil, fmt.Errorf("invalid base64 content: %w", err)
			}
		} else {
			data = []byte(content)
		}

		if err := m.provider.WriteFile(ctx, path, data); err != nil {
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

		return map[string]interface{}{
			"status":  "success",
			"entries": entries,
		}, nil

	case "search_files":
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}

		matches, err := m.provider.SearchFiles(ctx, pattern)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{
			"status":  "success",
			"matches": matches,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

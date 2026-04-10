package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(isCloud bool) *HybridFSMCP {
	var provider FileSystemProvider
	if isCloud {
		provider = &CloudFSProvider{}
	} else {
		provider = &LocalFSProvider{}
	}
	return &HybridFSMCP{provider: provider}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the content of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Write content to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "List entries in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Search for files by pattern.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if os.Getenv("OHC_MULTITENANT") == "true" && claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	// Fallback to empty claims for local mode if nil
	if claims == nil {
		claims = &auth.Claims{}
	}

	pathInter, ok := arguments["path"]
	if !ok {
		return nil, errors.New("missing path argument")
	}
	path, ok := pathInter.(string)
	if !ok {
		return nil, errors.New("path must be a string")
	}

	switch toolName {
	case "read_file":
		content, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(content)}, nil
	case "write_file":
		contentInter, ok := arguments["content"]
		if !ok {
			return nil, errors.New("missing content argument")
		}
		contentStr, ok := contentInter.(string)
		if !ok {
			return nil, errors.New("content must be a string")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(contentStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil
	case "search_files":
		patternInter, ok := arguments["pattern"]
		if !ok {
			return nil, errors.New("missing pattern argument")
		}
		pattern, ok := patternInter.(string)
		if !ok {
			return nil, errors.New("pattern must be a string")
		}
		entries, err := m.provider.SearchFiles(ctx, claims, path, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

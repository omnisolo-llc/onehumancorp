package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	IsLocal() bool
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

type HybridFSProxy struct {
	provider FileSystemProvider
}

func NewHybridFSProxy(provider FileSystemProvider) *HybridFSProxy {
	return &HybridFSProxy{
		provider: provider,
	}
}

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

func (m *HybridFSProxy) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

func (m *HybridFSProxy) resolvePath(claims *auth.Claims, path string) string {
	if m.provider.IsLocal() || claims == nil {
		return path
	}

	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	if strings.HasPrefix(cleanPath, claims.OrganizationID+"/") {
		return cleanPath
	}

	return fmt.Sprintf("%s/%s", claims.OrganizationID, cleanPath)
}

func (m *HybridFSProxy) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		scopedPath := m.resolvePath(claims, path)
		content, err := m.provider.ReadFile(ctx, scopedPath)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{
			"status": "success",
			"content": string(content),
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

		scopedPath := m.resolvePath(claims, path)
		err := m.provider.WriteFile(ctx, scopedPath, []byte(content))
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

		scopedPath := m.resolvePath(claims, path)
		files, err := m.provider.ListDir(ctx, scopedPath)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{
			"status": "success",
			"files": files,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

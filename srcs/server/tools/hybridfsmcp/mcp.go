package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	base string
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode.
type CloudFSProvider struct {
	base string
}

// NewFileSystemProvider creates the appropriate FileSystemProvider based on the environment.
func NewFileSystemProvider() FileSystemProvider {
	base := os.Getenv("OHC_FS_ROOT")
	if base == "" {
		base = os.TempDir()
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &CloudFSProvider{base: base}
	}
	return &LocalFSProvider{base: base}
}

// LocalFSProvider Implementation

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	if filepath.IsAbs(filepath.Clean(target)) {
		return "", errors.New("absolute paths are not allowed")
	}

	fullPath := filepath.Join(p.base, target)
	fullPath = filepath.Clean(fullPath)

	if fullPath != p.base && !strings.HasPrefix(fullPath, p.base+string(filepath.Separator)) {
		return "", errors.New("path traversal outside base directory is not allowed")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += string(filepath.Separator)
		}
		names = append(names, name)
	}

	return names, nil
}

// CloudFSProvider Implementation

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, target string) (string, error) {
	if claims == nil {
		return "", errors.New("unauthorized: missing claims in cloud mode")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization id in claims")
	}

	if filepath.IsAbs(filepath.Clean(target)) {
		return "", errors.New("absolute paths are not allowed")
	}

	tenantBase := filepath.Join(p.base, claims.OrganizationID)

	fullPath := filepath.Join(tenantBase, target)
	fullPath = filepath.Clean(fullPath)

	if fullPath != tenantBase && !strings.HasPrefix(fullPath, tenantBase+string(filepath.Separator)) {
		return "", errors.New("path traversal outside tenant base directory is not allowed")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += string(filepath.Separator)
		}
		names = append(names, name)
	}

	return names, nil
}

// HybridFSMCP Implementation

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

func (m *HybridFSMCP) ListTools() []Tool {
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
			Description: "Lists contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		pathObj, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, errors.New("path must be a string")
		}

		content, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content": string(content)}, nil

	case "write_file":
		pathObj, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, errors.New("path must be a string")
		}

		contentObj, ok := arguments["content"]
		if !ok {
			return nil, errors.New("missing content argument")
		}
		content, ok := contentObj.(string)
		if !ok {
			return nil, errors.New("content must be a string")
		}

		err := m.provider.WriteFile(ctx, claims, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		pathObj, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		path, ok := pathObj.(string)
		if !ok {
			return nil, errors.New("path must be a string")
		}

		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "entries": entries}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

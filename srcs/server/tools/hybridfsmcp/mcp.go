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

// FileSystemProvider defines the interface for hybrid file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for unified filesystem access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
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
			Description: "Lists files and directories in a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
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
		return map[string]interface{}{"status": "success", "content": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		if err := m.provider.WriteFile(ctx, path, []byte(content)); err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		items, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "items": items}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to a specific workspace directory.
func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{
		workspaceDir: workspaceDir,
	}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	if strings.Contains(reqPath, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	cleanPath := filepath.Clean("/" + reqPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	return filepath.Join(p.workspaceDir, cleanPath), nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure parent directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}
	var items []string
	for _, entry := range entries {
		items = append(items, entry.Name())
	}
	return items, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant isolation.
type CloudFSProvider struct {
	baseMountPath string
}

// NewCloudFSProvider creates a new CloudFSProvider scoped by tenant.
func NewCloudFSProvider(baseMountPath string) *CloudFSProvider {
	return &CloudFSProvider{
		baseMountPath: baseMountPath,
	}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}

	if strings.Contains(reqPath, "..") {
		return "", errors.New("directory traversal not allowed")
	}

	cleanPath := filepath.Clean("/" + reqPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	tenantDir := filepath.Join(p.baseMountPath, claims.OrganizationID)
	return filepath.Join(tenantDir, cleanPath), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure parent directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}
	var items []string
	for _, entry := range entries {
		items = append(items, entry.Name())
	}
	return items, nil
}

// ProviderFactory creates the appropriate FileSystemProvider based on environment.
func ProviderFactory(workspaceDir, baseMountPath string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(baseMountPath)
	}
	return NewLocalFSProvider(workspaceDir)
}

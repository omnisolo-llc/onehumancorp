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

// FileSystemProvider defines the interface for file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for standalone mode
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) securePath(target string) (string, error) {
	cleanPath := filepath.Clean(target)
	fullPath := filepath.Join(p.baseDir, cleanPath)
	if !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) && fullPath != p.baseDir {
		return "", errors.New("path traversal detected")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.securePath(path)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for multi-tenant cloud mode
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) securePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	cleanPath := filepath.Clean(target)
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanPath)

	if !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) && fullPath != tenantDir {
		return "", errors.New("path traversal detected")
	}

	// Ensure tenant directory exists
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		return "", fmt.Errorf("failed to create tenant directory: %w", err)
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// HybridFSMCP implements the MCP interface for file system access
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

// Tool represents an MCP tool definition
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools
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

// CallTool executes a tool by name
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		content, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{
			"status":  "success",
			"content": string(content),
		}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		err := m.provider.WriteFile(ctx, path, []byte(contentStr))
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

		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{
			"status":  "success",
			"entries": entries,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// Factory function to create appropriate provider based on mode
func NewFileSystemProvider(isMultiTenant bool, baseDir string) FileSystemProvider {
	if isMultiTenant {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

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

// FileSystemProvider abstracts file writing and reading logic
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, path string, query string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode
type LocalFSProvider struct {
	WorkspaceDir string
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	fullPath := filepath.Join(p.WorkspaceDir, cleanTarget)

	// Ensure the fullPath starts with the WorkspaceDir + separator, or is exactly WorkspaceDir
	cleanWorkspace := filepath.Clean(p.WorkspaceDir)
	if !strings.HasPrefix(fullPath, cleanWorkspace+string(filepath.Separator)) && fullPath != cleanWorkspace {
		return "", errors.New("access denied: path escapes workspace")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Create directory if it doesn't exist
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path string, query string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var files []string
	err = filepath.Walk(fullPath, func(walkPath string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), query) {
			relPath, err := filepath.Rel(p.WorkspaceDir, walkPath)
			if err == nil {
				files = append(files, relPath)
			}
		}
		return nil
	})

	return files, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode
type CloudFSProvider struct {
	BasePath string
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	cleanTarget := filepath.Clean(target)
	tenantPath := filepath.Join(p.BasePath, claims.OrganizationID)
	fullPath := filepath.Join(tenantPath, cleanTarget)

	// Ensure the fullPath starts with the tenantPath + separator, or is exactly tenantPath
	cleanTenantPath := filepath.Clean(tenantPath)
	if !strings.HasPrefix(fullPath, cleanTenantPath+string(filepath.Separator)) && fullPath != cleanTenantPath {
		return "", errors.New("access denied: path escapes tenant directory")
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Create directory if it doesn't exist
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path string, query string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	claims := auth.ClaimsFromContext(ctx)
	tenantPath := filepath.Join(p.BasePath, claims.OrganizationID)

	var files []string
	err = filepath.Walk(fullPath, func(walkPath string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), query) {
			relPath, err := filepath.Rel(tenantPath, walkPath)
			if err == nil {
				files = append(files, relPath)
			}
		}
		return nil
	})

	return files, err
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for database introspection.
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
			Description: "Reads a file from the file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file in the file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files in the file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "query": {"type": "string"}}, "required": ["path", "query"]}`,
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
		return map[string]interface{}{"status": "success", "data": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		err := m.provider.WriteFile(ctx, path, []byte(data))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		files, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "files": files}, nil
	case "search_files":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}

		files, err := m.provider.SearchFiles(ctx, path, query)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{"status": "success", "files": files}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// Ensure factory logic correctly instantiates the provider based on the OHC_MULTITENANT and OHC_STANDALONE modes.
func NewProvider(mode string, workspaceDir string) FileSystemProvider {
	if mode == "OHC_STANDALONE" {
		return &LocalFSProvider{WorkspaceDir: workspaceDir}
	}
	return &CloudFSProvider{BasePath: workspaceDir}
}

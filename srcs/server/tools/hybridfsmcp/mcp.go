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

// FileSystemProvider abstracts the underlying file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for hybrid file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{
		workspaceDir: filepath.Clean(workspaceDir),
	}
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(targetPath)
	rel, err := filepath.Rel(p.workspaceDir, filepath.Join(p.workspaceDir, cleanPath))
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}
	return filepath.Join(p.workspaceDir, cleanPath), nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode, scoping access by Tenant ID.
type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseStorageDir: filepath.Clean(baseStorageDir),
	}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, targetPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}
	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)

	cleanPath := filepath.Clean(targetPath)
	rel, err := filepath.Rel(tenantDir, filepath.Join(tenantDir, cleanPath))
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}
	return filepath.Join(tenantDir, cleanPath), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func NewHybridFSMCP() *HybridFSMCP {
	var provider FileSystemProvider

	if os.Getenv("OHC_STANDALONE") == "true" {
		workspace := os.Getenv("OHC_WORKSPACE_DIR")
		if workspace == "" {
			workspace = "/tmp/ohc_workspace"
		}
		provider = NewLocalFSProvider(workspace)
	} else {
		// Default to Cloud/Multitenant
		baseStorage := os.Getenv("OHC_BASE_STORAGE_DIR")
		if baseStorage == "" {
			baseStorage = "/tmp/ohc_cloud_storage"
		}
		provider = NewCloudFSProvider(baseStorage)
	}

	return &HybridFSMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read a file from the hybrid file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Write a file to the hybrid file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "List contents of a directory in the hybrid file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch toolName {
	case "read_file":
		pathObj, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		pathStr, ok := pathObj.(string)
		if !ok {
			return nil, errors.New("path must be a string")
		}
		data, err := m.provider.ReadFile(ctx, claims, pathStr)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"data": string(data)}, nil

	case "write_file":
		pathObj, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		pathStr, ok := pathObj.(string)
		if !ok {
			return nil, errors.New("path must be a string")
		}
		dataObj, ok := arguments["data"]
		if !ok {
			return nil, errors.New("missing data argument")
		}
		dataStr, ok := dataObj.(string)
		if !ok {
			return nil, errors.New("data must be a string")
		}
		err := m.provider.WriteFile(ctx, claims, pathStr, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		pathObj, ok := arguments["path"]
		if !ok {
			return nil, errors.New("missing path argument")
		}
		pathStr, ok := pathObj.(string)
		if !ok {
			return nil, errors.New("path must be a string")
		}
		names, err := m.provider.ListDir(ctx, claims, pathStr)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"files": names}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

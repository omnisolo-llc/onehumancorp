package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// FileSystemProvider abstracts the underlying file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for a local directory.
type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: baseDir}
}

func (l *LocalFSProvider) resolvePath(p string) (string, error) {
	absBase, err := filepath.Abs(l.BaseDir)
	if err != nil {
		return "", err
	}
	absTarget, err := filepath.Abs(filepath.Join(l.BaseDir, p))
	if err != nil {
		return "", err
	}
	// Path traversal protection
	if !strings.HasPrefix(absTarget, absBase+string(filepath.Separator)) && absTarget != absBase {
		return "", fmt.Errorf("path traversal denied")
	}
	return absTarget, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// CloudFSProvider implements FileSystemProvider with tenant scoping based on auth.Claims.
type CloudFSProvider struct {
	BaseVolumePath string
}

func NewCloudFSProvider(baseVolumePath string) *CloudFSProvider {
	return &CloudFSProvider{BaseVolumePath: baseVolumePath}
}

func (c *CloudFSProvider) resolvePath(ctx context.Context, p string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing or invalid tenant claims")
	}

	tenantDir := filepath.Join(c.BaseVolumePath, claims.OrganizationID)
	absBase, err := filepath.Abs(tenantDir)
	if err != nil {
		return "", err
	}
	absTarget, err := filepath.Abs(filepath.Join(tenantDir, p))
	if err != nil {
		return "", err
	}
	// Path traversal protection
	if !strings.HasPrefix(absTarget, absBase+string(filepath.Separator)) && absTarget != absBase {
		return "", fmt.Errorf("path traversal denied")
	}
	return absTarget, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// HybridFS MCP Server wrapping the provider
type HybridFS struct {
	provider FileSystemProvider
}

// Factory to create the provider based on mode
func NewFileSystemProvider(mode string, basePath string) FileSystemProvider {
	if mode == "OHC_MULTITENANT" {
		return NewCloudFSProvider(basePath)
	}
	// Default to local/standalone
	return NewLocalFSProvider(basePath)
}

func NewHybridFS(provider FileSystemProvider) *HybridFS {
	return &HybridFS{provider: provider}
}

// ListTools returns the tools supported by this MCP server.
func (s *HybridFS) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file",
			InputSchema: `{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file",
			InputSchema: `{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}},"required":["path","content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path",
			InputSchema: `{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (s *HybridFS) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	argsBytes, err := json.Marshal(arguments)
	if err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	switch toolName {
	case "read_file":
		return s.handleReadFile(ctx, argsBytes)
	case "write_file":
		return s.handleWriteFile(ctx, argsBytes)
	case "list_directory":
		return s.handleListDir(ctx, argsBytes)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (s *HybridFS) handleReadFile(ctx context.Context, args []byte) (interface{}, error) {
	var input struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	data, err := s.provider.ReadFile(ctx, input.Path)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"content": string(data),
	}, nil
}

func (s *HybridFS) handleWriteFile(ctx context.Context, args []byte) (interface{}, error) {
	var input struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	err := s.provider.WriteFile(ctx, input.Path, []byte(input.Content))
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"success": true,
	}, nil
}

func (s *HybridFS) handleListDir(ctx context.Context, args []byte) (interface{}, error) {
	var input struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	entries, err := s.provider.ListDir(ctx, input.Path)
	if err != nil {
		return nil, err
	}

	var items []string
	for _, entry := range entries {
		suffix := ""
		if entry.IsDir() {
			suffix = "/"
		}
		items = append(items, entry.Name()+suffix)
	}

	return map[string]interface{}{
		"items": items,
	}, nil
}

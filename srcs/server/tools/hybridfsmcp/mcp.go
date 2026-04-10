package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"io/fs"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// FileSystemProvider abstracts file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, path string, pattern string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for local standalone mode.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
	cleanBase := filepath.Clean(p.baseDir)
	cleanTarget := filepath.Clean(filepath.Join(cleanBase, targetPath))

	if !strings.HasPrefix(cleanTarget, cleanBase+string(filepath.Separator)) && cleanTarget != cleanBase {
		return "", errors.New("path escalation detected")
	}
	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.securePath(path)
	if err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	safeDir, err := p.securePath(dir)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(safeDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}

		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}
		if matched {
			rel, err := filepath.Rel(safeDir, path)
			if err != nil {
				return err
			}
			matches = append(matches, rel)
		}
		return nil
	})

	return matches, err
}

// CloudFSProvider implements FileSystemProvider for cloud multi-tenant mode.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) securePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanBase := filepath.Clean(tenantDir)
	cleanTarget := filepath.Clean(filepath.Join(cleanBase, targetPath))

	if !strings.HasPrefix(cleanTarget, cleanBase+string(filepath.Separator)) && cleanTarget != cleanBase {
		return "", errors.New("path escalation detected")
	}

	// Ensure tenant directory exists
	if err := os.MkdirAll(cleanBase, 0755); err != nil {
		return "", err
	}

	return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure parent directory exists
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	safeDir, err := p.securePath(ctx, dir)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(safeDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err // Or continue? Let's return error to be safe
		}
		if d.IsDir() {
			return nil
		}

		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}
		if matched {
			rel, err := filepath.Rel(safeDir, path)
			if err != nil {
				return err
			}
			matches = append(matches, rel)
		}
		return nil
	})

	return matches, err
}

// HybridFSInspectorMCP implements the MCP interface for hybrid file system access.
type HybridFSInspectorMCP struct {
	provider FileSystemProvider
}

// NewHybridFSInspectorMCP creates a new HybridFSInspectorMCP instance.
func NewHybridFSInspectorMCP(provider FileSystemProvider) *HybridFSInspectorMCP {
	return &HybridFSInspectorMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *HybridFSInspectorMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the hybrid file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes a file to the hybrid file system.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSInspectorMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.readFile(ctx, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		dataStr, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		return m.writeFile(ctx, path, []byte(dataStr))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDir(ctx, path)
	case "search_files":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		return m.searchFiles(ctx, path, pattern)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSInspectorMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status": "success",
		"data":   string(data),
	}, nil
}

func (m *HybridFSInspectorMCP) writeFile(ctx context.Context, path string, data []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, data)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status": "success",
	}, nil
}

func (m *HybridFSInspectorMCP) listDir(ctx context.Context, path string) (interface{}, error) {
	files, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status": "success",
		"files":  files,
	}, nil
}

func (m *HybridFSInspectorMCP) searchFiles(ctx context.Context, path string, pattern string) (interface{}, error) {
	files, err := m.provider.SearchFiles(ctx, path, pattern)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{
		"status": "success",
		"files":  files,
	}, nil
}

// Factory function
func NewFileSystemProvider() FileSystemProvider {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	if isStandalone {
		return NewLocalFSProvider("/tmp/ohc_workspace") // Or get from config
	}
	return NewCloudFSProvider("/mnt/pv/ohc_tenants") // Or get from config
}

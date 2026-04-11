package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
	SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	if baseDir == "" {
		baseDir = os.Getenv("OHC_FS_ROOT")
		if baseDir == "" {
			baseDir = os.TempDir()
		}
	}
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) IsLocal() bool { return true }

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.baseDir, target))
	if cleanTarget == p.baseDir || strings.HasPrefix(cleanTarget, p.baseDir+string(filepath.Separator)) {
		return cleanTarget, nil
	}
	return "", fmt.Errorf("access denied: path escapes base directory")
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	dir, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var infos []fs.FileInfo
	for _, d := range dir {
		info, err := d.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	fullPath, err := p.resolvePath(dir)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(fullPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			matched, _ := filepath.Match(pattern, d.Name())
			if matched || strings.Contains(d.Name(), pattern) {
				relPath, _ := filepath.Rel(p.baseDir, path)
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode with tenant isolation
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	if baseDir == "" {
		baseDir = os.Getenv("OHC_FS_ROOT")
		if baseDir == "" {
			baseDir = "/mnt/data/tenants"
		}
	}
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) IsLocal() bool { return false }

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanTarget := filepath.Clean(filepath.Join(tenantDir, target))
	if cleanTarget == tenantDir || strings.HasPrefix(cleanTarget, tenantDir+string(filepath.Separator)) {
		return cleanTarget, nil
	}
	return "", fmt.Errorf("access denied: path escapes tenant directory")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	dir, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var infos []fs.FileInfo
	for _, d := range dir {
		info, err := d.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	fullPath, err := p.resolveTenantPath(ctx, dir)
	if err != nil {
		return nil, err
	}

	tenantDir := filepath.Dir(fullPath)
	// get tenant dir from resolveTenantPath instead of re-building it to ensure it matches
	claims := auth.ClaimsFromContext(ctx)
	if claims != nil && claims.OrganizationID != "" {
		tenantDir = filepath.Join(p.baseDir, claims.OrganizationID)
	}

	var matches []string
	err = filepath.WalkDir(fullPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			matched, _ := filepath.Match(pattern, d.Name())
			if matched || strings.Contains(d.Name(), pattern) {
				relPath, _ := filepath.Rel(tenantDir, path)
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// FSFactory creates the appropriate FileSystemProvider based on environment
func FSFactory() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider("")
	}
	return NewLocalFSProvider("")
}

// HybridFSMCP implements the MCP interface for filesystem access
type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file at the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file at the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in the given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern in a directory.",
			InputSchema: `{"type": "object", "properties": {"dir": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["dir", "pattern"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	path, _ := arguments["path"].(string)

	switch toolName {
	case "read_file":
		if path == "" {
			return nil, fmt.Errorf("missing path")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content": string(data)}, nil

	case "write_file":
		if path == "" {
			return nil, fmt.Errorf("missing path")
		}
		content, _ := arguments["content"].(string)
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		if path == "" {
			path = "."
		}
		infos, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		var files []map[string]interface{}
		for _, info := range infos {
			files = append(files, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
			})
		}
		return map[string]interface{}{"status": "success", "files": files}, nil

	case "search_files":
		dir, _ := arguments["dir"].(string)
		if dir == "" {
			dir = "."
		}
		pattern, _ := arguments["pattern"].(string)
		if pattern == "" {
			return nil, fmt.Errorf("missing pattern")
		}
		matches, err := m.provider.SearchFiles(ctx, dir, pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "matches": matches}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

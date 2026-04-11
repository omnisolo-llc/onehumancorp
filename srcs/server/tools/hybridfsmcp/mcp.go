package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, dir, pattern string) ([]string, error)
}

type LocalFSProvider struct {
	rootDir string
}

func NewLocalFSProvider(rootDir string) *LocalFSProvider {
	if rootDir == "" {
		rootDir = "."
	}
	abs, err := filepath.Abs(rootDir)
	if err == nil {
		rootDir = abs
	}
	return &LocalFSProvider{rootDir: rootDir}
}

func (p *LocalFSProvider) resolve(path string) (string, error) {
	joined := filepath.Join(p.rootDir, path)
	abs, err := filepath.Abs(joined)
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(abs, p.rootDir+string(filepath.Separator)) && abs != p.rootDir {
		return "", errors.New("access denied: path outside root directory")
	}
	return abs, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	abs, err := p.resolve(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(abs)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	abs, err := p.resolve(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(abs), 0755); err != nil {
		return err
	}
	return os.WriteFile(abs, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	abs, err := p.resolve(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(abs)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir, pattern string) ([]string, error) {
	abs, err := p.resolve(dir)
	if err != nil {
		return nil, err
	}
	var res []string
	err = filepath.WalkDir(abs, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(p.rootDir, path)
			res = append(res, rel)
		}
		return nil
	})
	return res, err
}

type CloudFSProvider struct {
	baseVolume string
}

func NewCloudFSProvider(baseVolume string) *CloudFSProvider {
	if baseVolume == "" {
		baseVolume = "/tmp/ohc_cloud_volumes"
	}
	abs, err := filepath.Abs(baseVolume)
	if err == nil {
		baseVolume = abs
	}
	return &CloudFSProvider{baseVolume: baseVolume}
}

func (p *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant organization claims")
	}
	// Sanitize Org ID just in case
	safeOrgID := filepath.Base(claims.OrganizationID)
	return filepath.Join(p.baseVolume, safeOrgID), nil
}

func (p *CloudFSProvider) resolve(ctx context.Context, path string) (string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return "", err
	}
	joined := filepath.Join(tenantDir, path)
	abs, err := filepath.Abs(joined)
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(abs, tenantDir+string(filepath.Separator)) && abs != tenantDir {
		return "", errors.New("access denied: cross-tenant or outside tenant directory path")
	}
	return abs, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	abs, err := p.resolve(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(abs)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	abs, err := p.resolve(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(abs), 0755); err != nil {
		return err
	}
	return os.WriteFile(abs, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	abs, err := p.resolve(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(abs)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir, pattern string) ([]string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return nil, err
	}
	abs, err := p.resolve(ctx, dir)
	if err != nil {
		return nil, err
	}
	var res []string
	err = filepath.WalkDir(abs, func(pPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(tenantDir, pPath)
			res = append(res, rel)
		}
		return nil
	})
	return res, err
}

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
			Description: "Lists files and directories in a path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files by pattern in a directory.",
			InputSchema: `{"type": "object", "properties": {"dir": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["dir", "pattern"]}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path'")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return string(data), nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path'")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content'")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]string{"status": "success"}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path'")
		}
		res, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return res, nil
	case "search_files":
		dir, ok := arguments["dir"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'dir'")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern'")
		}
		res, err := m.provider.SearchFiles(ctx, dir, pattern)
		if err != nil {
			return nil, err
		}
		return res, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func ProviderFactory() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(os.Getenv("OHC_CLOUD_VOLUMES"))
	}
	return NewLocalFSProvider(os.Getenv("OHC_LOCAL_WORKSPACE"))
}

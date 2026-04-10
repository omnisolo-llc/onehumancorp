package hybridfsmcp

import (
	"context"
	"encoding/json"
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
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, query string) ([]string, error)
}

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSServer(baseDir string) *HybridFSMCP {
	var provider FileSystemProvider
	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider = NewCloudFSProvider(baseDir)
	} else {
		provider = NewLocalFSProvider(baseDir)
	}
	return &HybridFSMCP{provider: provider}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a query.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`),
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing path")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing path")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing content")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing path")
		}
		files, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"files": files}, nil
	case "search_files":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing query")
		}
		files, err := m.provider.SearchFiles(ctx, query)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"files": files}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	clean := filepath.Clean(reqPath)
	if filepath.IsAbs(clean) {
		return "", errors.New("absolute paths are not allowed")
	}
	if strings.HasPrefix(clean, "..") {
		return "", errors.New("paths outside base directory are not allowed")
	}
	return filepath.Join(p.baseDir, clean), nil
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
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	var res []string
	err := filepath.WalkDir(p.baseDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			if os.IsNotExist(err) {
				return nil
			}
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), query) {
			rel, _ := filepath.Rel(p.baseDir, path)
			res = append(res, rel)
		}
		return nil
	})
	return res, err
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}
	clean := filepath.Clean(reqPath)
	if filepath.IsAbs(clean) {
		return "", errors.New("absolute paths are not allowed")
	}
	if strings.HasPrefix(clean, "..") {
		return "", errors.New("paths outside base directory are not allowed")
	}
	return filepath.Join(p.baseDir, claims.OrganizationID, clean), nil
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
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing organization ID")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	var res []string
	err := filepath.WalkDir(tenantDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			if os.IsNotExist(err) {
				return nil
			}
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), query) {
			rel, _ := filepath.Rel(tenantDir, path)
			res = append(res, rel)
		}
		return nil
	})
	return res, err
}

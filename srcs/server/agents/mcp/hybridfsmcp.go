package mcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts the file system access.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements file system access for Standalone mode, bounded to a specific directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	absPath := filepath.Join(p.baseDir, path)
	rel, err := filepath.Rel(p.baseDir, absPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(filepath.ToSlash(rel), "../") {
		return "", errors.New("path outside base directory")
	}
	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements file system access for Cloud mode, scoped by tenant.
type CloudFSProvider struct {
	tenantRootDir string
}

func NewCloudFSProvider(tenantRootDir string) (*CloudFSProvider, error) {
	absRoot, err := filepath.Abs(tenantRootDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{tenantRootDir: absRoot}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	tenantBase := filepath.Join(p.tenantRootDir, claims.OrganizationID)
	absPath := filepath.Join(tenantBase, path)

	rel, err := filepath.Rel(tenantBase, absPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(filepath.ToSlash(rel), "../") {
		return "", errors.New("path outside tenant directory")
	}
	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// HybridFSServer exposes filesystem operations as MCP tools.
type HybridFSServer struct {
	provider FileSystemProvider
}

// Tool Definition reused from other files or defined here for completeness if not exported globally
// Using a local struct to avoid dependency on statesyncmcp.Tool if it's not exposed properly in the package
type HybridTool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

func NewHybridFSServer(isStandalone bool, baseDir string) (*HybridFSServer, error) {
	var provider FileSystemProvider
	var err error

	if isStandalone {
		provider, err = NewLocalFSProvider(baseDir)
	} else {
		provider, err = NewCloudFSProvider(baseDir)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSServer{provider: provider}, nil
}

func (s *HybridFSServer) ListTools() []HybridTool {
	return []HybridTool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "List the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

func (s *HybridFSServer) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	pathInter, ok := arguments["path"]
	if !ok {
		return nil, errors.New("missing required argument: path")
	}
	pathStr, ok := pathInter.(string)
	if !ok {
		return nil, errors.New("invalid path argument type")
	}

	switch toolName {
	case "read_file":
		content, err := s.provider.ReadFile(ctx, pathStr)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(content)}, nil

	case "write_file":
		contentInter, ok := arguments["content"]
		if !ok {
			return nil, errors.New("missing required argument: content")
		}
		contentStr, ok := contentInter.(string)
		if !ok {
			return nil, errors.New("invalid content argument type")
		}
		err := s.provider.WriteFile(ctx, pathStr, []byte(contentStr))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		entries, err := s.provider.ListDir(ctx, pathStr)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

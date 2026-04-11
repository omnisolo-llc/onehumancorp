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

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]os.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider for standalone local access
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to a specific directory
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, target))
	// Prevent path traversal vulnerabilities
	if cleanPath != p.baseDir && !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path outside workspace")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	infos := make([]os.FileInfo, 0, len(entries))
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries where stat fails
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider for tenant-scoped cloud access
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, target))

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path outside tenant scope")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	infos := make([]os.FileInfo, 0, len(entries))
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// FileSystemMCP implements the MCP interface for file operations
type FileSystemMCP struct {
	provider FileSystemProvider
}

// NewFileSystemMCP creates a new FileSystemMCP instance based on the environment mode.
func NewFileSystemMCP(mode string, baseDir string) *FileSystemMCP {
	var provider FileSystemProvider
	if mode == "true" { // OHC_MULTITENANT == "true"
		provider = NewCloudFSProvider(baseDir)
	} else {
		provider = NewLocalFSProvider(baseDir)
	}

	return &FileSystemMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *FileSystemMCP) ListTools() []Tool {
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
			Description: "Lists files and directories under a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *FileSystemMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
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
			"status": "success",
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
		path := "."
		if p, ok := arguments["path"].(string); ok {
			path = p
		}

		infos, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}

		var results []map[string]interface{}
		for _, info := range infos {
			results = append(results, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
			})
		}

		return map[string]interface{}{
			"status": "success",
			"files":  results,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

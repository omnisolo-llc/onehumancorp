package hybridfsmcp

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations
type FileSystemProvider interface {
	ReadFile(path string) ([]byte, error)
	WriteFile(path string, content []byte) error
	ListDir(path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace dir.
type LocalFSProvider struct {
	WorkspaceRoot string
}

func (l *LocalFSProvider) resolveAndVerifyPath(path string) (string, error) {
	absRoot, err := filepath.Abs(l.WorkspaceRoot)
	if err != nil {
		return "", err
	}

	// Join the path with root and get absolute path
	targetPath := filepath.Join(absRoot, path)
	absTarget, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	cleanAbsTarget := filepath.Clean(absTarget)
	cleanAbsRoot := filepath.Clean(absRoot)

	if !(strings.HasPrefix(cleanAbsTarget, cleanAbsRoot+string(filepath.Separator)) || cleanAbsTarget == cleanAbsRoot) {
		return "", fmt.Errorf("path traversal attempt detected")
	}
	return cleanAbsTarget, nil
}

func (l *LocalFSProvider) ReadFile(path string) ([]byte, error) {
	safePath, err := l.resolveAndVerifyPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (l *LocalFSProvider) WriteFile(path string, content []byte) error {
	safePath, err := l.resolveAndVerifyPath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, content, 0644)
}

func (l *LocalFSProvider) ListDir(path string) ([]string, error) {
	safePath, err := l.resolveAndVerifyPath(path)
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

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode, scoping access by tenant ID.
type CloudFSProvider struct {
	MountRoot string
	Claims    *auth.Claims
}

func (c *CloudFSProvider) resolveAndVerifyPath(path string) (string, error) {
	if c.Claims == nil || c.Claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}

	// Scope the root to the tenant's directory within the MountRoot
	tenantRoot := filepath.Join(c.MountRoot, c.Claims.OrganizationID)
	absRoot, err := filepath.Abs(tenantRoot)
	if err != nil {
		return "", err
	}

	targetPath := filepath.Join(absRoot, path)
	absTarget, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	cleanAbsTarget := filepath.Clean(absTarget)
	cleanAbsRoot := filepath.Clean(absRoot)

	if !(strings.HasPrefix(cleanAbsTarget, cleanAbsRoot+string(filepath.Separator)) || cleanAbsTarget == cleanAbsRoot) {
		return "", fmt.Errorf("path traversal attempt detected")
	}
	return cleanAbsTarget, nil
}

func (c *CloudFSProvider) ReadFile(path string) ([]byte, error) {
	safePath, err := c.resolveAndVerifyPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (c *CloudFSProvider) WriteFile(path string, content []byte) error {
	safePath, err := c.resolveAndVerifyPath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, content, 0644)
}

func (c *CloudFSProvider) ListDir(path string) ([]string, error) {
	safePath, err := c.resolveAndVerifyPath(path)
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

// ProviderFactory creates the appropriate FileSystemProvider based on environment.
func ProviderFactory(claims *auth.Claims, defaultWorkspace string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		mountRoot := os.Getenv("OHC_CLOUD_FS_MOUNT")
		if mountRoot == "" {
			mountRoot = "/mnt/tenant_data"
		}
		return &CloudFSProvider{
			MountRoot: mountRoot,
			Claims:    claims,
		}
	}
	// Fallback to Standalone/Local mode
	workspace := os.Getenv("OHC_LOCAL_WORKSPACE")
	if workspace == "" {
		workspace = defaultWorkspace
	}
	if workspace == "" {
		workspace = "." // Current directory fallback
	}
	return &LocalFSProvider{
		WorkspaceRoot: workspace,
	}
}

// Tool definitions for MCP
type ReadFileInput struct {
	Path string `json:"path"`
}

type WriteFileInput struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirInput struct {
	Path string `json:"path"`
}

// HybridFSMCP implements the MCP interface for the Hybrid File System Proxy.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
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
			Description: "Lists files and directories in a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a query. (Not implemented yet)",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx interface{}, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, err := m.provider.ReadFile(path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(content)}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil

	case "search_files":
		return nil, fmt.Errorf("search_files not implemented yet")

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

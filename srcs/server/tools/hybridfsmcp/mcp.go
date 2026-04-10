package hybridfsmcp

import (
    "context"
    "encoding/json"
    "errors"
    "fmt"
    "os"
    "path/filepath"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
    ReadFile(ctx context.Context, path string) ([]byte, error)
    WriteFile(ctx context.Context, path string, content []byte) error
    ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
    WorkspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
    absPath, err := filepath.Abs(workspaceDir)
    if err != nil {
        return nil, err
    }
    return &LocalFSProvider{WorkspaceDir: absPath}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
    cleanTarget := filepath.Clean(target)
    if filepath.IsAbs(cleanTarget) {
        return "", errors.New("absolute paths are not allowed")
    }
    fullPath := filepath.Join(p.WorkspaceDir, cleanTarget)
    rel, err := filepath.Rel(p.WorkspaceDir, fullPath)
    if err != nil {
        return "", err
    }
    if rel == ".." || strings.HasPrefix(rel, ".." + string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
    fullPath, err := p.resolvePath(path)
    if err != nil {
        return err
    }
    dir := filepath.Dir(fullPath)
    if err := os.MkdirAll(dir, 0755); err != nil {
        return err
    }
    return os.WriteFile(fullPath, content, 0644)
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
    var names []string
    for _, entry := range entries {
        names = append(names, entry.Name())
    }
    return names, nil
}

type CloudFSProvider struct {
    BaseDir string
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
    absPath, err := filepath.Abs(baseDir)
    if err != nil {
        return nil, err
    }
    return &CloudFSProvider{BaseDir: absPath}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil || claims.OrganizationID == "" {
        return "", errors.New("unauthorized: missing claims or organization ID")
    }

    tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)

    cleanTarget := filepath.Clean(target)
    if filepath.IsAbs(cleanTarget) {
        return "", errors.New("absolute paths are not allowed")
    }

    fullPath := filepath.Join(tenantDir, cleanTarget)
    rel, err := filepath.Rel(tenantDir, fullPath)
    if err != nil {
        return "", err
    }
    if rel == ".." || strings.HasPrefix(rel, ".." + string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
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

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
    fullPath, err := p.resolvePath(ctx, path)
    if err != nil {
        return err
    }
    dir := filepath.Dir(fullPath)
    if err := os.MkdirAll(dir, 0755); err != nil {
        return err
    }
    return os.WriteFile(fullPath, content, 0644)
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
    var names []string
    for _, entry := range entries {
        names = append(names, entry.Name())
    }
    return names, nil
}

type Tool struct {
    Name        string          `json:"name"`
    Description string          `json:"description"`
    InputSchema json.RawMessage `json:"inputSchema"`
}

type HybridFSMCP struct {
    provider FileSystemProvider
}

func NewHybridFSMCP(isStandalone bool, baseDir string) (*HybridFSMCP, error) {
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
    return &HybridFSMCP{provider: provider}, nil
}

func (m *HybridFSMCP) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads a file from the hybrid file system.",
            InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
        },
        {
            Name:        "write_file",
            Description: "Writes content to a file in the hybrid file system.",
            InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
        },
        {
            Name:        "list_directory",
            Description: "Lists contents of a directory in the hybrid file system.",
            InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
        },
    }
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
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
        return map[string]interface{}{"content": string(content)}, nil
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
        return map[string]interface{}{"status": "success"}, nil
    case "list_directory":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path' argument")
        }
        entries, err := m.provider.ListDir(ctx, path)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"entries": entries}, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

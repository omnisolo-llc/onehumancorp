package mcp

import (
    "context"
    "os"
    "path/filepath"
    "strings"
    "fmt"
)

type FileSystemProvider interface {
    ReadFile(ctx context.Context, path string, claims map[string]interface{}) ([]byte, error)
    WriteFile(ctx context.Context, path string, data []byte, claims map[string]interface{}) error
    ListDir(ctx context.Context, path string, claims map[string]interface{}) ([]string, error)
}

type LocalFSProvider struct {
    workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
    return &LocalFSProvider{workspaceDir: workspaceDir}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
    absPath, err := filepath.Abs(filepath.Join(p.workspaceDir, path))
    if err != nil {
        return "", err
    }
    if !strings.HasPrefix(absPath, p.workspaceDir+string(filepath.Separator)) && absPath != p.workspaceDir {
        return "", fmt.Errorf("path escapes workspace bounds: %s", path)
    }
    return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string, claims map[string]interface{}) ([]byte, error) {
    absPath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(absPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte, claims map[string]interface{}) error {
    absPath, err := p.resolvePath(path)
    if err != nil {
        return err
    }
    return os.WriteFile(absPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string, claims map[string]interface{}) ([]string, error) {
    absPath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(absPath)
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
    baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
    return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) resolvePath(path string, claims map[string]interface{}) (string, error) {
    tenantID, ok := claims["tenant_id"].(string)
    if !ok || tenantID == "" {
        return "", fmt.Errorf("missing or invalid tenant_id in claims")
    }
    tenantDir := filepath.Join(p.baseDir, tenantID)
    absPath, err := filepath.Abs(filepath.Join(tenantDir, path))
    if err != nil {
        return "", err
    }
    if !strings.HasPrefix(absPath, tenantDir+string(filepath.Separator)) && absPath != tenantDir {
        return "", fmt.Errorf("path escapes tenant bounds: %s", path)
    }
    return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string, claims map[string]interface{}) ([]byte, error) {
    absPath, err := p.resolvePath(path, claims)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(absPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte, claims map[string]interface{}) error {
    absPath, err := p.resolvePath(path, claims)
    if err != nil {
        return err
    }
    if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
        return err
    }
    return os.WriteFile(absPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string, claims map[string]interface{}) ([]string, error) {
    absPath, err := p.resolvePath(path, claims)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(absPath)
    if err != nil {
        return nil, err
    }
    var names []string
    for _, entry := range entries {
        names = append(names, entry.Name())
    }
    return names, nil
}

func NewFSProvider(mode string, dir string) FileSystemProvider {
    if mode == "OHC_MULTITENANT" {
        return NewCloudFSProvider(dir)
    }
    return NewLocalFSProvider(dir)
}

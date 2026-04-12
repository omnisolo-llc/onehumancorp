package hybridfsmcp

import (
    "context"
    "fmt"
    "os"
    "path/filepath"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
    ReadFile(ctx context.Context, path string) ([]byte, error)
    WriteFile(ctx context.Context, path string, data []byte) error
    ListDir(ctx context.Context, path string) ([]string, error)
    SearchFiles(ctx context.Context, path, pattern string) ([]string, error)
}

type LocalFSProvider struct {
    basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
    return &LocalFSProvider{basePath: basePath}
}

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
    cleanPath := filepath.Clean(targetPath)
    if filepath.IsAbs(cleanPath) {
        return "", fmt.Errorf("absolute paths are not allowed")
    }

    fullPath := filepath.Join(p.basePath, cleanPath)
    rel, err := filepath.Rel(p.basePath, fullPath)
    if err != nil {
        return "", err
    }
    if strings.HasPrefix(rel, "..") {
        return "", fmt.Errorf("path traversal not allowed")
    }
    return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    fullPath, err := p.securePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    fullPath, err := p.securePath(path)
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
    fullPath, err := p.securePath(path)
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

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
    fullPath, err := p.securePath(path)
    if err != nil {
        return nil, err
    }
    var matches []string
    err = filepath.WalkDir(fullPath, func(path string, d os.DirEntry, err error) error {
        if err == nil && !d.IsDir() && strings.Contains(d.Name(), pattern) {
            rel, _ := filepath.Rel(p.basePath, path)
            matches = append(matches, rel)
        }
        return nil
    })
    return matches, err
}

type CloudFSProvider struct {
    basePath string
}

func NewCloudFSProvider(basePath string) *CloudFSProvider {
    return &CloudFSProvider{basePath: basePath}
}

func (p *CloudFSProvider) securePath(ctx context.Context, targetPath string) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil || claims.OrganizationID == "" {
        return "", fmt.Errorf("unauthorized: missing organization ID")
    }

    cleanPath := filepath.Clean(targetPath)
    if filepath.IsAbs(cleanPath) {
        return "", fmt.Errorf("absolute paths are not allowed")
    }

    tenantBasePath := filepath.Join(p.basePath, claims.OrganizationID)

    fullPath := filepath.Join(tenantBasePath, cleanPath)
    rel, err := filepath.Rel(tenantBasePath, fullPath)
    if err != nil {
        return "", err
    }
    if strings.HasPrefix(rel, "..") {
        return "", fmt.Errorf("path traversal not allowed")
    }
    return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    fullPath, err := p.securePath(ctx, path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    fullPath, err := p.securePath(ctx, path)
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
    fullPath, err := p.securePath(ctx, path)
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

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
    fullPath, err := p.securePath(ctx, path)
    if err != nil {
        return nil, err
    }
    var matches []string
    tenantBasePath := filepath.Join(p.basePath, auth.ClaimsFromContext(ctx).OrganizationID)
    err = filepath.WalkDir(fullPath, func(path string, d os.DirEntry, err error) error {
        if err == nil && !d.IsDir() && strings.Contains(d.Name(), pattern) {
            rel, _ := filepath.Rel(tenantBasePath, path)
            matches = append(matches, rel)
        }
        return nil
    })
    return matches, err
}

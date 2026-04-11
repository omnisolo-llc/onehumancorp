package hybridfsmcp

import (
    "context"
    "errors"
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
}

type LocalFSProvider struct {
    workspaceRoot string
}

func (p *LocalFSProvider) sanitizePath(path string) (string, error) {
    if strings.Contains(path, "..") {
        return "", errors.New("directory traversal not allowed")
    }
    cleanPath := filepath.Clean("/" + path)
    cleanPath = strings.TrimPrefix(cleanPath, "/")
    fullPath := filepath.Join(p.workspaceRoot, cleanPath)
    if !strings.HasPrefix(fullPath, p.workspaceRoot) {
        return "", errors.New("path outside workspace root")
    }
    return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    safePath, err := p.sanitizePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    safePath, err := p.sanitizePath(path)
    if err != nil {
        return err
    }
    dir := filepath.Dir(safePath)
    if err := os.MkdirAll(dir, 0755); err != nil {
        return err
    }
    return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
    safePath, err := p.sanitizePath(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(safePath)
    if err != nil {
        if errors.Is(err, fs.ErrNotExist) {
            return []string{}, nil
        }
        return nil, err
    }
    var names []string
    for _, entry := range entries {
        names = append(names, entry.Name())
    }
    return names, nil
}

type CloudFSProvider struct {
    volumeRoot string
}

func (p *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil || claims.OrganizationID == "" {
        return "", errors.New("unauthorized: missing claims or organization ID")
    }
    return filepath.Join(p.volumeRoot, claims.OrganizationID), nil
}

func (p *CloudFSProvider) sanitizePath(ctx context.Context, path string) (string, error) {
    if strings.Contains(path, "..") {
        return "", errors.New("directory traversal not allowed")
    }
    tenantDir, err := p.getTenantDir(ctx)
    if err != nil {
        return "", err
    }
    cleanPath := filepath.Clean("/" + path)
    cleanPath = strings.TrimPrefix(cleanPath, "/")
    fullPath := filepath.Join(tenantDir, cleanPath)
    if !strings.HasPrefix(fullPath, tenantDir) {
        return "", errors.New("path outside tenant root")
    }
    return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    safePath, err := p.sanitizePath(ctx, path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    safePath, err := p.sanitizePath(ctx, path)
    if err != nil {
        return err
    }
    dir := filepath.Dir(safePath)
    if err := os.MkdirAll(dir, 0755); err != nil {
        return err
    }
    return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
    safePath, err := p.sanitizePath(ctx, path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(safePath)
    if err != nil {
        if errors.Is(err, fs.ErrNotExist) {
            return []string{}, nil
        }
        return nil, err
    }
    var names []string
    for _, entry := range entries {
        names = append(names, entry.Name())
    }
    return names, nil
}

func NewFileSystemProvider(isStandalone bool, rootDir string) FileSystemProvider {
    if isStandalone {
        return &LocalFSProvider{workspaceRoot: rootDir}
    }
    return &CloudFSProvider{volumeRoot: rootDir}
}

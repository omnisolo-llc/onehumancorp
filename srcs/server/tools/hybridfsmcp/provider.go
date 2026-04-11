package hybridfsmcp

import (
    "context"
    "fmt"
    "io/fs"
    "os"
    "path/filepath"
    "strings"
)

type FileSystemProvider interface {
    ReadFile(ctx context.Context, path string) ([]byte, error)
    WriteFile(ctx context.Context, path string, data []byte) error
    ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

type LocalFSProvider struct {
    baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
    return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) securePath(path string) (string, error) {
    cleanPath := filepath.Clean(path)
    fullPath := filepath.Join(p.baseDir, cleanPath)

    // Ensure boundary check is directory-aware
    rel, err := filepath.Rel(p.baseDir, fullPath)
    if err != nil || strings.HasPrefix(rel, "..") || strings.HasPrefix(rel, "/") {
        return "", fmt.Errorf("access denied: %s is outside base directory", path)
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

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
    fullPath, err := p.securePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadDir(fullPath)
}

type CloudFSProvider struct {
    baseDir  string
    tenantID string
}

func NewCloudFSProvider(baseDir, tenantID string) *CloudFSProvider {
    return &CloudFSProvider{
        baseDir:  filepath.Clean(baseDir),
        tenantID: tenantID,
    }
}

func (p *CloudFSProvider) securePath(path string) (string, error) {
    if p.tenantID == "" {
        return "", fmt.Errorf("tenant ID is required")
    }
    tenantDir := filepath.Join(p.baseDir, p.tenantID)
    cleanPath := filepath.Clean(path)
    fullPath := filepath.Join(tenantDir, cleanPath)

    // Ensure boundary check is directory-aware
    rel, err := filepath.Rel(tenantDir, fullPath)
    if err != nil || strings.HasPrefix(rel, "..") || strings.HasPrefix(rel, "/") {
        return "", fmt.Errorf("access denied: %s is outside tenant directory", path)
    }

    return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    fullPath, err := p.securePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
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

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
    fullPath, err := p.securePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadDir(fullPath)
}

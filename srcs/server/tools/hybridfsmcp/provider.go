package hybridfsmcp

import (
    "context"
    "errors"
    "os"
    "path/filepath"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

// FileInfo describes a file or directory.
type FileInfo struct {
    Name  string
    IsDir bool
    Size  int64
}

// FileSystemProvider defines the unified interface for file operations.
type FileSystemProvider interface {
    ReadFile(ctx context.Context, path string) ([]byte, error)
    WriteFile(ctx context.Context, path string, data []byte) error
    ListDir(ctx context.Context, path string) ([]FileInfo, error)
    IsLocal() bool
}

// LocalFSProvider implements file operations for Standalone Mode.
type LocalFSProvider struct {
    basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
    absPath, _ := filepath.Abs(basePath)
    return &LocalFSProvider{basePath: absPath}
}

func (p *LocalFSProvider) IsLocal() bool { return true }

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
    fullPath := filepath.Join(p.basePath, reqPath)
    rel, err := filepath.Rel(p.basePath, fullPath)
    if err != nil || rel == ".." || strings.HasPrefix(rel, "../") {
        return "", errors.New("path traversal detected")
    }
    return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    safePath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    safePath, err := p.resolvePath(path)
    if err != nil {
        return err
    }
    if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
        return err
    }
    return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
    safePath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(safePath)
    if err != nil {
        return nil, err
    }
    var result []FileInfo
    for _, entry := range entries {
        info, err := entry.Info()
        if err != nil {
            continue
        }
        result = append(result, FileInfo{
            Name:  entry.Name(),
            IsDir: entry.IsDir(),
            Size:  info.Size(),
        })
    }
    return result, nil
}

// CloudFSProvider implements tenant-scoped file operations for Cloud Mode.
type CloudFSProvider struct {
    basePath string
}

func NewCloudFSProvider(basePath string) *CloudFSProvider {
    absPath, _ := filepath.Abs(basePath)
    return &CloudFSProvider{basePath: absPath}
}

func (p *CloudFSProvider) IsLocal() bool { return false }

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return "", errors.New("unauthorized: missing claims")
    }
    tenantPath := filepath.Join(p.basePath, claims.OrganizationID)
    fullPath := filepath.Join(tenantPath, reqPath)
    rel, err := filepath.Rel(tenantPath, fullPath)
    if err != nil || rel == ".." || strings.HasPrefix(rel, "../") {
        return "", errors.New("path traversal detected")
    }
    return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    safePath, err := p.resolvePath(ctx, path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    safePath, err := p.resolvePath(ctx, path)
    if err != nil {
        return err
    }
    if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
        return err
    }
    return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
    safePath, err := p.resolvePath(ctx, path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(safePath)
    if err != nil {
        return nil, err
    }
    var result []FileInfo
    for _, entry := range entries {
        info, err := entry.Info()
        if err != nil {
            continue
        }
        result = append(result, FileInfo{
            Name:  entry.Name(),
            IsDir: entry.IsDir(),
            Size:  info.Size(),
        })
    }
    return result, nil
}

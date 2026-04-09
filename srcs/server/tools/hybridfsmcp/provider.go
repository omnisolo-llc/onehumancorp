package hybridfsmcp

import (
    "context"
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
    ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
    SearchFiles(ctx context.Context, dir, pattern string) ([]string, error)
}

type LocalFSProvider struct {
    baseDir string
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
    cleanTarget := filepath.Clean(filepath.Join(p.baseDir, target))
    if !strings.HasPrefix(cleanTarget, filepath.Clean(p.baseDir)) {
        return "", fmt.Errorf("path %s is outside base directory", target)
    }
    return cleanTarget, nil
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
    os.MkdirAll(filepath.Dir(fullPath), 0755)
    return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
    fullPath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(fullPath)
    if err != nil {
        return nil, err
    }
    var infos []fs.FileInfo
    for _, e := range entries {
        info, err := e.Info()
        if err == nil {
            infos = append(infos, info)
        }
    }
    return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir, pattern string) ([]string, error) {
    fullPath, err := p.resolvePath(dir)
    if err != nil {
        return nil, err
    }
    var matches []string
    err = filepath.WalkDir(fullPath, func(path string, d fs.DirEntry, err error) error {
        if err != nil {
            return err
        }
        if !d.IsDir() && strings.Contains(d.Name(), pattern) {
            rel, _ := filepath.Rel(p.baseDir, path)
            matches = append(matches, rel)
        }
        return nil
    })
    return matches, err
}

type CloudFSProvider struct {
    baseVolume string
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, target string) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return "", fmt.Errorf("unauthorized: missing claims")
    }
    tenantDir := filepath.Join(p.baseVolume, claims.OrganizationID)
    cleanTarget := filepath.Clean(filepath.Join(tenantDir, target))
    if !strings.HasPrefix(cleanTarget, filepath.Clean(tenantDir)) {
        return "", fmt.Errorf("path %s is outside tenant directory", target)
    }
    return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    fullPath, err := p.resolveTenantPath(ctx, path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    fullPath, err := p.resolveTenantPath(ctx, path)
    if err != nil {
        return err
    }
    os.MkdirAll(filepath.Dir(fullPath), 0755)
    return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
    fullPath, err := p.resolveTenantPath(ctx, path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(fullPath)
    if err != nil {
        return nil, err
    }
    var infos []fs.FileInfo
    for _, e := range entries {
        info, err := e.Info()
        if err == nil {
            infos = append(infos, info)
        }
    }
    return infos, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir, pattern string) ([]string, error) {
    fullPath, err := p.resolveTenantPath(ctx, dir)
    if err != nil {
        return nil, err
    }
    var matches []string
    tenantDir := filepath.Join(p.baseVolume, auth.ClaimsFromContext(ctx).OrganizationID)
    err = filepath.WalkDir(fullPath, func(path string, d fs.DirEntry, err error) error {
        if err != nil {
            return err
        }
        if !d.IsDir() && strings.Contains(d.Name(), pattern) {
            rel, _ := filepath.Rel(tenantDir, path)
            matches = append(matches, rel)
        }
        return nil
    })
    return matches, err
}

func NewFileSystemProvider(isStandalone bool, baseDir string) FileSystemProvider {
    if isStandalone {
        return &LocalFSProvider{baseDir: baseDir}
    }
    return &CloudFSProvider{baseVolume: baseDir}
}

package hybridfsmcp

import (
    "context"
    "fmt"
    "os"
    "path/filepath"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

type LocalFSProvider struct {
    basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
    return &LocalFSProvider{basePath: basePath}
}

func (p *LocalFSProvider) IsLocal() bool {
    return true
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
    cleanPath := filepath.Clean(filepath.Join(p.basePath, path))

    // Append separator to ensure exact boundary check, preventing prefix hijacking
    // e.g. /base/tenant1 vs /base/tenant10
    baseWithSep := filepath.Clean(p.basePath) + string(filepath.Separator)
    cleanWithSep := cleanPath
    if cleanPath != filepath.Clean(p.basePath) {
        cleanWithSep += string(filepath.Separator)
    }

    if !strings.HasPrefix(cleanWithSep, baseWithSep) && cleanPath != filepath.Clean(p.basePath) {
        return "", fmt.Errorf("path escapes base directory: %s", path)
    }
    return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
    fullPath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
    fullPath, err := p.resolvePath(path)
    if err != nil {
        return err
    }
    err = os.MkdirAll(filepath.Dir(fullPath), 0755)
    if err != nil {
        return err
    }
    return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
    fullPath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(fullPath)
    if err != nil {
        return nil, err
    }
    var files []string
    for _, entry := range entries {
        files = append(files, entry.Name())
    }
    return files, nil
}

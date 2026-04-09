package hybridfsmcp

import (
    "context"
    "fmt"
    "os"
    "path/filepath"
    "strings"
)

type LocalFSProvider struct {
    BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
    return &LocalFSProvider{BaseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
    cleanBase := filepath.Clean(p.BaseDir)
    resolved := filepath.Clean(filepath.Join(cleanBase, targetPath))
    if resolved != cleanBase && !strings.HasPrefix(resolved, cleanBase+string(filepath.Separator)) {
        return "", fmt.Errorf("path %s is outside base directory", targetPath)
    }
    return resolved, nil
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

    // Ensure dir exists
    if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
        return err
    }

    return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
    fullPath, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }

    entries, err := os.ReadDir(fullPath)
    if err != nil {
        return nil, err
    }

    var infos []FileInfo
    for _, entry := range entries {
        info, err := entry.Info()
        if err != nil {
            continue
        }
        infos = append(infos, FileInfo{
            Name:  entry.Name(),
            IsDir: entry.IsDir(),
            Size:  info.Size(),
        })
    }
    return infos, nil
}

package hybridfsmcp

import (
    "context"
    "errors"
    "os"
    "path/filepath"
    "strings"
)

type LocalFSProvider struct {
    basePath string
}

func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
    abs, err := filepath.Abs(basePath)
    if err != nil {
        return nil, err
    }
    return &LocalFSProvider{basePath: abs}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
    cleanTarget := filepath.Clean("/" + target)
    cleanTarget = strings.TrimPrefix(cleanTarget, "/")
    full := filepath.Join(p.basePath, cleanTarget)
    if !strings.HasPrefix(full, p.basePath) {
        return "", errors.New("directory traversal attempt")
    }
    return full, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    full, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(full)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    full, err := p.resolvePath(path)
    if err != nil {
        return err
    }
    if err := os.MkdirAll(filepath.Dir(full), 0755); err != nil {
        return err
    }
    return os.WriteFile(full, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]map[string]interface{}, error) {
    full, err := p.resolvePath(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(full)
    if err != nil {
        return nil, err
    }
    var result []map[string]interface{}
    for _, e := range entries {
        info, err := e.Info()
        if err != nil {
            continue
        }
        result = append(result, map[string]interface{}{
            "name":  e.Name(),
            "isDir": e.IsDir(),
            "size":  info.Size(),
        })
    }
    return result, nil
}

func (p *LocalFSProvider) IsLocal() bool {
    return true
}

package hybridfsmcp

import (
    "context"
    "fmt"
    "os"
    "path/filepath"
    "strings"
)

type LocalFSProvider struct {
    WorkspaceDir string
}

func (p *LocalFSProvider) ensureBound(path string) (string, error) {
    cleanPath := filepath.Clean(path)
    if !filepath.IsAbs(cleanPath) {
        cleanPath = filepath.Join(p.WorkspaceDir, cleanPath)
    }

    baseDir := p.WorkspaceDir
    if !strings.HasSuffix(baseDir, string(filepath.Separator)) {
        baseDir += string(filepath.Separator)
    }

    if !strings.HasPrefix(cleanPath, baseDir) && cleanPath != p.WorkspaceDir {
        return "", fmt.Errorf("path bounds error: %s is outside %s", cleanPath, baseDir)
    }
    return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    cleanPath, err := p.ensureBound(path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(cleanPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
    cleanPath, err := p.ensureBound(path)
    if err != nil {
        return err
    }
    if err := os.MkdirAll(filepath.Dir(cleanPath), 0755); err != nil {
        return err
    }
    return os.WriteFile(cleanPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
    cleanPath, err := p.ensureBound(path)
    if err != nil {
        return nil, err
    }
    entries, err := os.ReadDir(cleanPath)
    if err != nil {
        return nil, err
    }
    var names []string
    for _, entry := range entries {
        names = append(names, entry.Name())
    }
    return names, nil
}

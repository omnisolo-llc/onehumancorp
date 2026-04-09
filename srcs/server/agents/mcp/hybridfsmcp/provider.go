package hybridfsmcp

import (
    "io/fs"
    "os"
    "path/filepath"
    "strings"
    "errors"
)

type FileSystemProvider interface {
    ReadFile(path string) ([]byte, error)
    WriteFile(path string, content []byte) error
    ListDir(path string) ([]fs.DirEntry, error)
}

type LocalFSProvider struct {
    WorkspaceDir string
}

func (p *LocalFSProvider) resolve(path string) (string, error) {
    abs, err := filepath.Abs(filepath.Join(p.WorkspaceDir, path))
    if err != nil {
        return "", err
    }
    if !strings.HasPrefix(abs, filepath.Clean(p.WorkspaceDir)+string(filepath.Separator)) && abs != filepath.Clean(p.WorkspaceDir) {
        return "", errors.New("access denied: path outside workspace")
    }
    return abs, nil
}

func (p *LocalFSProvider) ReadFile(path string) ([]byte, error) {
    resolved, err := p.resolve(path)
    if err != nil { return nil, err }
    return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(path string, content []byte) error {
    resolved, err := p.resolve(path)
    if err != nil { return err }
    // Ensure directory exists
    if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil { return err }
    return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(path string) ([]fs.DirEntry, error) {
    resolved, err := p.resolve(path)
    if err != nil { return nil, err }
    return os.ReadDir(resolved)
}

type CloudFSProvider struct {
    TenantID string
    BaseDir  string
}

func (p *CloudFSProvider) resolve(path string) (string, error) {
    tenantDir := filepath.Join(p.BaseDir, p.TenantID)
    abs, err := filepath.Abs(filepath.Join(tenantDir, path))
    if err != nil {
        return "", err
    }
    if !strings.HasPrefix(abs, filepath.Clean(tenantDir)+string(filepath.Separator)) && abs != filepath.Clean(tenantDir) {
        return "", errors.New("access denied: path outside tenant scope")
    }
    return abs, nil
}

func (p *CloudFSProvider) ReadFile(path string) ([]byte, error) {
    resolved, err := p.resolve(path)
    if err != nil { return nil, err }
    return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(path string, content []byte) error {
    resolved, err := p.resolve(path)
    if err != nil { return err }
    // Ensure directory exists
    if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil { return err }
    return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(path string) ([]fs.DirEntry, error) {
    resolved, err := p.resolve(path)
    if err != nil { return nil, err }
    return os.ReadDir(resolved)
}

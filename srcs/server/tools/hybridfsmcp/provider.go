package hybridfsmcp

import (
    "fmt"
    "os"
    "path/filepath"
    "strings"
)

type FileSystemProvider interface {
    ReadFile(path string) ([]byte, error)
    WriteFile(path string, data []byte) error
    ListDir(path string) ([]string, error)
}

type LocalFSProvider struct {
    BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
    return &LocalFSProvider{BaseDir: baseDir}
}

func (p *LocalFSProvider) checkPath(pPath string) error {
    absPath, err := filepath.Abs(filepath.Join(p.BaseDir, pPath))
    if err != nil {
        return err
    }
    baseAbs, _ := filepath.Abs(p.BaseDir)
    if !(strings.HasPrefix(absPath, baseAbs+string(filepath.Separator)) || absPath == baseAbs) {
        return fmt.Errorf("path out of bounds")
    }
    return nil
}

func (p *LocalFSProvider) ReadFile(path string) ([]byte, error) {
    if err := p.checkPath(path); err != nil { return nil, err }
    return os.ReadFile(filepath.Join(p.BaseDir, path))
}

func (p *LocalFSProvider) WriteFile(path string, data []byte) error {
    if err := p.checkPath(path); err != nil { return err }
    return os.WriteFile(filepath.Join(p.BaseDir, path), data, 0644)
}

func (p *LocalFSProvider) ListDir(path string) ([]string, error) {
    if err := p.checkPath(path); err != nil { return nil, err }
    entries, err := os.ReadDir(filepath.Join(p.BaseDir, path))
    if err != nil { return nil, err }
    var res []string
    for _, e := range entries { res = append(res, e.Name()) }
    return res, nil
}

type CloudFSProvider struct {
    TenantID string
}

func NewCloudFSProvider(tenantID string) *CloudFSProvider {
    return &CloudFSProvider{TenantID: tenantID}
}

func (p *CloudFSProvider) ReadFile(path string) ([]byte, error) {
    return nil, fmt.Errorf("not implemented")
}
func (p *CloudFSProvider) WriteFile(path string, data []byte) error {
    return fmt.Errorf("not implemented")
}
func (p *CloudFSProvider) ListDir(path string) ([]string, error) {
    return nil, fmt.Errorf("not implemented")
}

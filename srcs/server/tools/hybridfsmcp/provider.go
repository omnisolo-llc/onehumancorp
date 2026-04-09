package hybridfsmcp

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(path string, claims *auth.Claims) ([]byte, error)
	WriteFile(path string, data []byte, claims *auth.Claims) error
	ListDir(path string, claims *auth.Claims) ([]fs.FileInfo, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider() *LocalFSProvider {
	baseDir := os.Getenv("OHC_WORKSPACE_DIR")
	if baseDir == "" {
		baseDir = "./workspace"
	}
	baseDir, _ = filepath.Abs(baseDir)
	os.MkdirAll(baseDir, 0700)
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, reqPath))
	if cleanPath == p.baseDir || strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", fmt.Errorf("path escapes base directory")
}

func (p *LocalFSProvider) ReadFile(path string, claims *auth.Claims) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(path string, data []byte, claims *auth.Claims) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0600)
}

func (p *LocalFSProvider) ListDir(path string, claims *auth.Claims) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	infos := make([]fs.FileInfo, len(entries))
	for i, e := range entries {
		info, err := e.Info()
		if err != nil {
			return nil, err
		}
		infos[i] = info
	}
	return infos, nil
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider() *CloudFSProvider {
	baseDir := os.Getenv("OHC_TENANT_PV_DIR")
	if baseDir == "" {
		baseDir = "/tenant_data"
	}
	baseDir, _ = filepath.Abs(baseDir)
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) resolvePath(reqPath string, claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing claims or organization ID")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	os.MkdirAll(tenantDir, 0700)

	cleanPath := filepath.Clean(filepath.Join(tenantDir, reqPath))
	if cleanPath == tenantDir || strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", fmt.Errorf("path escapes tenant directory")
}

func (p *CloudFSProvider) ReadFile(path string, claims *auth.Claims) ([]byte, error) {
	fullPath, err := p.resolvePath(path, claims)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(path string, data []byte, claims *auth.Claims) error {
	fullPath, err := p.resolvePath(path, claims)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0600)
}

func (p *CloudFSProvider) ListDir(path string, claims *auth.Claims) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(path, claims)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	infos := make([]fs.FileInfo, len(entries))
	for i, e := range entries {
		info, err := e.Info()
		if err != nil {
			return nil, err
		}
		infos[i] = info
	}
	return infos, nil
}

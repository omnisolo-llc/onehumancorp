package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absBase, _ := filepath.Abs(baseDir)
	return &LocalFSProvider{baseDir: absBase}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, path))
	if cleanPath != p.baseDir && !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("directory traversal blocked")
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	absBase, _ := filepath.Abs(baseDir)
	return &CloudFSProvider{baseDir: absBase}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, path))
	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("tenant boundary traversal blocked")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
	if basePath == "" {
		basePath = "/tmp/ohc_workspace"
	}
	return &LocalFSProvider{basePath: basePath}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.basePath, path))
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(filepath.Clean(absPath), filepath.Clean(p.basePath)+string(filepath.Separator)) && filepath.Clean(absPath) != filepath.Clean(p.basePath) {
		return "", errors.New("path traversal detected")
	}
	return absPath, nil
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode.
type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider(basePath string) *CloudFSProvider {
	if basePath == "" {
		basePath = "/mnt/tenant_volumes"
	}
	return &CloudFSProvider{basePath: basePath}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}

	// Sanitize tenant ID
	tenantID := filepath.Clean(claims.OrganizationID)
	if strings.Contains(tenantID, "/") || strings.Contains(tenantID, "\\") {
		return "", errors.New("invalid tenant ID")
	}

	tenantBase := filepath.Join(p.basePath, tenantID)
	absPath, err := filepath.Abs(filepath.Join(tenantBase, path))
	if err != nil {
		return "", err
	}

	if !strings.HasPrefix(filepath.Clean(absPath), filepath.Clean(tenantBase)+string(filepath.Separator)) && filepath.Clean(absPath) != filepath.Clean(tenantBase) {
		return "", errors.New("path traversal detected")
	}
	return absPath, nil
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.DirEntry, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

func NewProvider(standalone bool, basePath string) FileSystemProvider {
	if standalone {
		return NewLocalFSProvider(basePath)
	}
	return NewCloudFSProvider(basePath)
}

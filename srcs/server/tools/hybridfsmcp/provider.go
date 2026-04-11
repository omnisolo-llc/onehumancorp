package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for unified file system access.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: baseDir}
}

func (p *LocalFSProvider) IsLocal() bool { return true }

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Join(p.BaseDir, path)
	fullPath = filepath.Clean(fullPath)
	if !strings.HasPrefix(fullPath, filepath.Clean(p.BaseDir)+string(filepath.Separator)) && fullPath != filepath.Clean(p.BaseDir) {
		return "", fmt.Errorf("path escapes base directory")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant isolation.
type CloudFSProvider struct {
	BaseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseDir: baseDir}
}

func (p *CloudFSProvider) IsLocal() bool { return false }

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing claims or organization ID")
	}
	tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, path)
	fullPath = filepath.Clean(fullPath)
	if !strings.HasPrefix(fullPath, filepath.Clean(tenantDir)+string(filepath.Separator)) && fullPath != filepath.Clean(tenantDir) {
		return "", fmt.Errorf("path escapes tenant directory")
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error) {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

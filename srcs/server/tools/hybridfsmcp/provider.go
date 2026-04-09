package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string, claims *auth.Claims) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte, claims *auth.Claims) error
	ListDir(ctx context.Context, path string, claims *auth.Claims) ([]os.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, scoped to a specific directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absPath, _ := filepath.Abs(filepath.Clean(baseDir))
	return &LocalFSProvider{baseDir: absPath}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.baseDir, path))
	if err != nil {
		return "", err
	}
	cleanAbsPath := filepath.Clean(absPath)
	if !strings.HasPrefix(cleanAbsPath, p.baseDir+string(filepath.Separator)) && cleanAbsPath != p.baseDir {
		return "", fmt.Errorf("path traversal denied")
	}
	return cleanAbsPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string, claims *auth.Claims) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte, claims *auth.Claims) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string, claims *auth.Claims) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode, scoped per tenant.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	absPath, _ := filepath.Abs(filepath.Clean(baseDir))
	return &CloudFSProvider{baseDir: absPath}
}

func (p *CloudFSProvider) resolvePath(path string, claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)

	// Ensure tenant directory exists
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		return "", fmt.Errorf("failed to create tenant directory: %w", err)
	}

	absPath, err := filepath.Abs(filepath.Join(tenantDir, path))
	if err != nil {
		return "", err
	}

	cleanAbsPath := filepath.Clean(absPath)
	if !strings.HasPrefix(cleanAbsPath, tenantDir+string(filepath.Separator)) && cleanAbsPath != tenantDir {
		return "", fmt.Errorf("path traversal denied")
	}
	return cleanAbsPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string, claims *auth.Claims) ([]byte, error) {
	fullPath, err := p.resolvePath(path, claims)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte, claims *auth.Claims) error {
	fullPath, err := p.resolvePath(path, claims)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string, claims *auth.Claims) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(path, claims)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

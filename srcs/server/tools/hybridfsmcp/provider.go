package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider bounds file operations to a specific workspace directory
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.baseDir, target))
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal attempt: %s", target)
	}
	return fullPath, nil
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

	// Create directory structure if it doesn't exist
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

// CloudFSProvider delegates to a local or volume-backed FS provider, scoping paths by Tenant
type CloudFSProvider struct {
	delegate FileSystemProvider
}

func NewCloudFSProvider(delegate FileSystemProvider) *CloudFSProvider {
	return &CloudFSProvider{delegate: delegate}
}

func (p *CloudFSProvider) getTenantScope(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized or missing organization ID in context")
	}
	return claims.OrganizationID, nil
}

func (p *CloudFSProvider) sanitizePath(path string) (string, error) {
	if strings.Contains(path, "..") {
		return "", fmt.Errorf("invalid path: %s", path)
	}
	cleanPath := filepath.Clean(filepath.Join("/", path))
	if !strings.HasPrefix(cleanPath, "/") || cleanPath == "" {
		return "", fmt.Errorf("invalid path: %s", path)
	}
	return strings.TrimPrefix(cleanPath, "/"), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	orgID, err := p.getTenantScope(ctx)
	if err != nil {
		return nil, err
	}

	safePath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}

	scopedPath := filepath.Join(orgID, safePath)
	return p.delegate.ReadFile(ctx, scopedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	orgID, err := p.getTenantScope(ctx)
	if err != nil {
		return err
	}

	safePath, err := p.sanitizePath(path)
	if err != nil {
		return err
	}

	scopedPath := filepath.Join(orgID, safePath)
	return p.delegate.WriteFile(ctx, scopedPath, data)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	orgID, err := p.getTenantScope(ctx)
	if err != nil {
		return nil, err
	}

	safePath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}

	scopedPath := filepath.Join(orgID, safePath)
	return p.delegate.ListDir(ctx, scopedPath)
}

// NewFileSystemProvider creates a FileSystemProvider based on environment constraints
func NewFileSystemProvider(baseDir string) FileSystemProvider {
	local := NewLocalFSProvider(baseDir)
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(local)
	}
	return local
}

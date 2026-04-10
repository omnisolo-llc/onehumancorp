package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for unified file system operations.
type FileSystemProvider interface {
	// IsLocal returns true if the provider is a standalone local provider (vs cloud-scoped).
	IsLocal() bool
	// ReadFile reads the content of a file.
	ReadFile(ctx context.Context, path string) ([]byte, error)
	// WriteFile writes content to a file.
	WriteFile(ctx context.Context, path string, content []byte) error
	// ListDir lists contents of a directory.
	ListDir(ctx context.Context, path string) ([]os.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider for standalone local access.
type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider() *LocalFSProvider {
	basePath := os.Getenv("OHC_FS_ROOT")
	if basePath == "" {
		basePath = os.TempDir()
	}
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		absPath = basePath
	}
	return &LocalFSProvider{basePath: absPath}
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.basePath, target))
	if err != nil {
		return "", err
	}

	// Path traversal protection
	if absTarget == p.basePath || strings.HasPrefix(absTarget, p.basePath+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", errors.New("access denied: path escapes workspace root")
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var infos []os.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider for tenant-isolated cloud access.
type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider() *CloudFSProvider {
	basePath := os.Getenv("OHC_FS_ROOT")
	if basePath == "" {
		basePath = "/mnt/cloud-volumes"
	}
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		absPath = basePath
	}
	return &CloudFSProvider{basePath: absPath}
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid claims for cloud fs")
	}

	tenantBase := filepath.Join(p.basePath, claims.OrganizationID)
	absTenantBase, err := filepath.Abs(tenantBase)
	if err != nil {
		return "", err
	}

	absTarget, err := filepath.Abs(filepath.Join(absTenantBase, target))
	if err != nil {
		return "", err
	}

	// Path traversal protection to ensure scoped within tenant directory
	if absTarget == absTenantBase || strings.HasPrefix(absTarget, absTenantBase+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", errors.New("access denied: path escapes tenant boundary")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var infos []os.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

// GetProvider instantiates the correct provider based on env
func GetProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider()
	}
	return NewLocalFSProvider()
}

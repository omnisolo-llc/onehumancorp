package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the unified interface for file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded to a workspace.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// resolvePath ensures the given path is within the base directory to prevent traversal.
func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	if filepath.IsAbs(target) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", target)
	}

	fullPath := filepath.Join(p.baseDir, filepath.Clean(target))

	// Prevent directory traversal
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes base directory: %s", target)
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip if we can't get info
		}
		infos = append(infos, info)
	}

	return infos, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, scoped by tenant ID.
// For this proxy, we simulate tenant PVs by using a root storage directory and
// appending the tenant ID to create a chroot-like environment per tenant.
type CloudFSProvider struct {
	rootDir string
}

// NewCloudFSProvider creates a new CloudFSProvider using rootDir as the base for all tenants.
func NewCloudFSProvider(rootDir string) *CloudFSProvider {
	return &CloudFSProvider{
		rootDir: filepath.Clean(rootDir),
	}
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// resolvePath ensures the path is scoped to the tenant's subdirectory.
func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant ID in context")
	}

	if filepath.IsAbs(target) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", target)
	}

	tenantBase := filepath.Join(p.rootDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantBase, filepath.Clean(target))

	// Prevent traversal outside the tenant's base directory
	if fullPath != tenantBase && !strings.HasPrefix(fullPath, tenantBase+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes tenant directory: %s", target)
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}

	return infos, nil
}

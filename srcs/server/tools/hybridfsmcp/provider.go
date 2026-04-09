package hybridfsmcp

import (
	"context"
	"errors"

	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for the local file system
// with strict path bounding to a specific base directory.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

// resolvePath resolves the given path against the base directory and
// ensures it does not escape the base directory bounds.
func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	// Construct absolute path based on base directory
	fullPath := filepath.Join(p.baseDir, filepath.Clean(target))

	// Ensure the resolved path starts with the base directory
	if !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) && fullPath != p.baseDir {
		return "", errors.New("path escapes base directory")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure parent directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}

	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		// Use relative paths to abstract baseDir away from caller
		relPath, _ := filepath.Rel(p.baseDir, filepath.Join(resolvedPath, entry.Name()))
		results = append(results, relPath)
	}

	return results, nil
}

// CloudFSProvider implements FileSystemProvider for the cloud file system
// with tenant scoping using context claims.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

// resolvePath resolves the given path against the tenant's scoped directory.
func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}

	// Scope base directory by tenant ID
	tenantBaseDir := filepath.Join(p.baseDir, claims.OrganizationID)

	// Clean and join the path
	fullPath := filepath.Join(tenantBaseDir, filepath.Clean(target))

	// Ensure the resolved path stays within the tenant's directory
	if !strings.HasPrefix(fullPath, tenantBaseDir+string(filepath.Separator)) && fullPath != tenantBaseDir {
		return "", errors.New("path escapes tenant directory")
	}

	return fullPath, nil
}

// Ensure the tenant base directory is stripped from output paths
func (p *CloudFSProvider) stripTenantPrefix(ctx context.Context, fullPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}

	tenantBaseDir := filepath.Join(p.baseDir, claims.OrganizationID)
	relPath, err := filepath.Rel(tenantBaseDir, fullPath)
	if err != nil {
		return "", err
	}
	return relPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}

	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		relPath, _ := p.stripTenantPrefix(ctx, filepath.Join(resolvedPath, entry.Name()))
		results = append(results, relPath)
	}

	return results, nil
}

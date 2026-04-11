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

// FileSystemProvider abstracts the hybrid file system logic.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, backed by the local filesystem.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.baseDir, target))

	// Append separator to baseDir for strict prefix checking
	baseDirWithSep := p.baseDir
	if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
		baseDirWithSep += string(filepath.Separator)
	}

	if !strings.HasPrefix(fullPath, baseDirWithSep) && fullPath != p.baseDir {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
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

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		results = append(results, entry.Name())
	}
	return results, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, tenant-scoped.
type CloudFSProvider struct {
	storageRoot string // This is a virtual root, like a persistent volume mount point
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(storageRoot string) *CloudFSProvider {
	return &CloudFSProvider{
		storageRoot: filepath.Clean(storageRoot),
	}
}

func (p *CloudFSProvider) resolveTenantPath(claims *auth.Claims, target string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization claims")
	}

	// Create the tenant's bounded workspace inside the storage root
	tenantRoot := filepath.Join(p.storageRoot, claims.OrganizationID)
	tenantRoot = filepath.Clean(tenantRoot)

	fullPath := filepath.Clean(filepath.Join(tenantRoot, target))

	tenantRootWithSep := tenantRoot
	if !strings.HasSuffix(tenantRootWithSep, string(filepath.Separator)) {
		tenantRootWithSep += string(filepath.Separator)
	}

	if !strings.HasPrefix(fullPath, tenantRootWithSep) && fullPath != tenantRoot {
		return "", errors.New("path traversal detected or cross-tenant access attempted")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolveTenantPath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.resolveTenantPath(claims, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolveTenantPath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return []string{}, nil // Tenant directory might not exist yet
		}
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		results = append(results, entry.Name())
	}
	return results, nil
}

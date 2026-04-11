package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// FileInfo provides metadata about a file or directory.
type FileInfo struct {
	Name         string
	Size         int64
	IsDir        bool
	LastModified time.Time
}

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for the local file system.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: baseDir,
	}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.baseDir, path))

	// Ensure the path does not escape the base directory
	// In Go, when verifying that a path does not escape a base directory using strings.HasPrefix(fullPath, baseDir), always ensure you append a trailing filepath.Separator to the baseDir before the check to prevent partial directory name vulnerabilities (e.g., /tmp/foo unintentionally matching /tmp/foobar).
	baseDirWithSep := p.baseDir
	if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
		baseDirWithSep += string(filepath.Separator)
	}

	if !strings.HasPrefix(fullPath, baseDirWithSep) && fullPath != p.baseDir {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}

// ReadFile reads the entire contents of a file.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

// WriteFile writes data to a file, creating it if it doesn't exist.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

// ListDir lists the contents of a directory.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip if we can't get info
		}
		infos = append(infos, FileInfo{
			Name:         entry.Name(),
			Size:         info.Size(),
			IsDir:        entry.IsDir(),
			LastModified: info.ModTime(),
		})
	}
	return infos, nil
}

// IsLocal returns true for LocalFSProvider.
func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider with tenant scoping (simulated via base directory).
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to baseDir.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: baseDir,
	}
}

func (p *CloudFSProvider) resolvePath(tenantID, path string) (string, error) {
	if tenantID == "" {
		return "", errors.New("unauthorized: missing tenant ID")
	}

	tenantDir := filepath.Clean(filepath.Join(p.baseDir, tenantID))
	fullPath := filepath.Clean(filepath.Join(tenantDir, path))

	tenantDirWithSep := tenantDir
	if !strings.HasSuffix(tenantDirWithSep, string(filepath.Separator)) {
		tenantDirWithSep += string(filepath.Separator)
	}

	if !strings.HasPrefix(fullPath, tenantDirWithSep) && fullPath != tenantDir {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}

// ReadFile reads the entire contents of a file for the tenant.
func (p *CloudFSProvider) ReadFile(ctx context.Context, tenantID, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(tenantID, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

// WriteFile writes data to a file for the tenant, creating it if it doesn't exist.
func (p *CloudFSProvider) WriteFile(ctx context.Context, tenantID, path string, data []byte) error {
	fullPath, err := p.resolvePath(tenantID, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

// ListDir lists the contents of a directory for the tenant.
func (p *CloudFSProvider) ListDir(ctx context.Context, tenantID, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(tenantID, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip if we can't get info
		}
		infos = append(infos, FileInfo{
			Name:         entry.Name(),
			Size:         info.Size(),
			IsDir:        entry.IsDir(),
			LastModified: info.ModTime(),
		})
	}
	return infos, nil
}

// IsLocal returns false for CloudFSProvider.
func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// CloudToFSProviderAdapter adapts CloudFSProvider to FileSystemProvider, extracting the tenant ID from the context.
// In a real system, the tenant ID would likely be extracted from the auth.Claims in the context.
type CloudToFSProviderAdapter struct {
	cloudProvider *CloudFSProvider
	getTenantID   func(ctx context.Context) string
}

// NewCloudToFSProviderAdapter creates a new CloudToFSProviderAdapter.
func NewCloudToFSProviderAdapter(cloudProvider *CloudFSProvider, getTenantID func(ctx context.Context) string) *CloudToFSProviderAdapter {
	return &CloudToFSProviderAdapter{
		cloudProvider: cloudProvider,
		getTenantID:   getTenantID,
	}
}

// ReadFile calls ReadFile on the underlying CloudFSProvider with the tenant ID from the context.
func (a *CloudToFSProviderAdapter) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return a.cloudProvider.ReadFile(ctx, a.getTenantID(ctx), path)
}

// WriteFile calls WriteFile on the underlying CloudFSProvider with the tenant ID from the context.
func (a *CloudToFSProviderAdapter) WriteFile(ctx context.Context, path string, data []byte) error {
	return a.cloudProvider.WriteFile(ctx, a.getTenantID(ctx), path, data)
}

// ListDir calls ListDir on the underlying CloudFSProvider with the tenant ID from the context.
func (a *CloudToFSProviderAdapter) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	return a.cloudProvider.ListDir(ctx, a.getTenantID(ctx), path)
}

// IsLocal delegates to the underlying CloudFSProvider.
func (a *CloudToFSProviderAdapter) IsLocal() bool {
	return a.cloudProvider.IsLocal()
}

package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, targetPath string) ([]byte, error)
	WriteFile(ctx context.Context, targetPath string, content []byte) error
	ListDir(ctx context.Context, targetPath string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded to a specific directory.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the given base directory.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	if filepath.IsAbs(filepath.Clean(targetPath)) {
		return "", errors.New("absolute paths are not allowed")
	}

	fullPath := filepath.Join(p.baseDir, filepath.Clean(targetPath))

	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("path escapes base directory bounds")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, targetPath string) ([]byte, error) {
	fullPath, err := p.resolvePath(targetPath)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, targetPath string, content []byte) error {
	fullPath, err := p.resolvePath(targetPath)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, targetPath string) ([]string, error) {
	fullPath, err := p.resolvePath(targetPath)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		names = append(names, name)
	}

	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, scoping paths by tenant ID.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider, using the given base directory to store tenant data.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseDir: absBase}, nil
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, targetPath string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", errors.New("unauthorized: missing organization ID in context")
	}

	if filepath.IsAbs(filepath.Clean(targetPath)) {
		return "", errors.New("absolute paths are not allowed")
	}

	tenantDir := filepath.Join(p.baseDir, orgID)
	fullPath := filepath.Join(tenantDir, filepath.Clean(targetPath))

	if fullPath != tenantDir && !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("path escapes tenant directory bounds")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, targetPath string) ([]byte, error) {
	fullPath, err := p.resolveTenantPath(ctx, targetPath)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, targetPath string, content []byte) error {
	fullPath, err := p.resolveTenantPath(ctx, targetPath)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, targetPath string) ([]string, error) {
	fullPath, err := p.resolveTenantPath(ctx, targetPath)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		names = append(names, name)
	}

	return names, nil
}

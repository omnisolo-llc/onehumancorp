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

// CloudFSProvider implements FileSystemProvider with tenant isolation.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to the given base directory.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseDir: absBase}, nil
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization claims")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	// Ensure tenant directory exists
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		return "", err
	}

	absPath, err := filepath.Abs(filepath.Join(tenantDir, path))
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(tenantDir, absPath)
	if err != nil || strings.HasPrefix(rel, "..") || rel == ".." {
		return "", errors.New("access denied: path escapes tenant directory")
	}
	return absPath, nil
}

// ReadFile reads a file from the tenant's filesystem.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

// WriteFile writes data to a file on the tenant's filesystem.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte, perm fs.FileMode) error {
	absPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, perm)
}

// ListDir lists the contents of a directory on the tenant's filesystem.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	absPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

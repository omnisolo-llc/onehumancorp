package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for the cloud, scoping access by tenant ID.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

// ReadFile reads a file from the tenant's isolated directory.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, path)

	if fullPath != tenantDir && !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) {
		return nil, errors.New("path traversal detected")
	}

	return os.ReadFile(fullPath)
}

// WriteFile writes a file to the tenant's isolated directory.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return errors.New("unauthorized: missing claims or organization ID")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, path)

	if fullPath != tenantDir && !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) {
		return errors.New("path traversal detected")
	}

	if filepath.Clean(path) == "." || filepath.Clean(path) == "/" {
		return errors.New("cannot write to root directory")
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

// ListDir lists the contents of a directory within the tenant's isolated directory.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, path)

	if fullPath != tenantDir && !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) {
		return nil, errors.New("path traversal detected")
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}

	return names, nil
}

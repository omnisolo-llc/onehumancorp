package hybridfsmcp

import (
	"context"
	"errors"
	"path/filepath"
	"strings"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

var (
	ErrUnauthorized = errors.New("unauthorized: missing or invalid tenant claims")
)

// CloudFSProvider implements FileSystemProvider for a multi-tenant cloud environment.
// It wraps a LocalFSProvider, but scopes all operations dynamically based on the
// OrganizationID found in the context claims.
type CloudFSProvider struct {
	baseStorageDir string
}

// NewCloudFSProvider creates a new CloudFSProvider with the given base storage directory.
func NewCloudFSProvider(baseStorageDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseStorageDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{
		baseStorageDir: absBase,
	}, nil
}

// getTenantProvider returns a properly scoped LocalFSProvider for the current tenant.
func (p *CloudFSProvider) getTenantProvider(ctx context.Context) (*LocalFSProvider, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, ErrUnauthorized
	}

	// Ensure the organization ID doesn't contain path traversal characters
	if strings.Contains(claims.OrganizationID, "..") || strings.Contains(claims.OrganizationID, string(filepath.Separator)) {
		return nil, ErrUnauthorized
	}

	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)

	// Create the provider which will enforce boundaries
	return NewLocalFSProvider(tenantDir)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return tenantProvider.ReadFile(ctx, path)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return err
	}
	return tenantProvider.WriteFile(ctx, path, data)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return tenantProvider.ListDir(ctx, path)
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return tenantProvider.SearchFiles(ctx, path, pattern)
}

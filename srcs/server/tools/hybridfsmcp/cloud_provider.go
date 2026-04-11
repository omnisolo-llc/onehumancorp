package hybridfsmcp

import (
	"context"
	"errors"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for the cloud, scoping paths by tenant.
type CloudFSProvider struct {
	baseProvider FileSystemProvider
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseProvider FileSystemProvider) *CloudFSProvider {
	return &CloudFSProvider{
		baseProvider: baseProvider,
	}
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims for cloud provider")
	}

	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	return filepath.Join(claims.OrganizationID, cleanPath), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return p.baseProvider.ReadFile(ctx, tenantPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	return p.baseProvider.WriteFile(ctx, tenantPath, data)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return p.baseProvider.ListDir(ctx, tenantPath)
}

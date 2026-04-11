package hybridfsmcp

import (
	"context"
	"errors"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: baseDir,
	}
}

func (p *CloudFSProvider) getTenantProvider(ctx context.Context) (FileSystemProvider, error) {
	tenantID := auth.OrganizationIDFromContext(ctx)
	if tenantID == "" {
		return nil, errors.New("unauthorized: missing tenant organization ID")
	}

	// We create a new LocalFSProvider securely bound only to the tenant's subdirectory.
	// This prevents path traversal into other tenant directories, as LocalFSProvider itself enforces bounds.
	tenantDir := filepath.Join(p.baseDir, tenantID)
	return NewLocalFSProvider(tenantDir), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	provider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return provider.ReadFile(ctx, path)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	provider, err := p.getTenantProvider(ctx)
	if err != nil {
		return err
	}
	return provider.WriteFile(ctx, path, content)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	provider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return provider.ListDir(ctx, path)
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	provider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	// The new local provider handles search natively and bounded to its local dir.
	return provider.SearchFiles(ctx, pattern)
}

package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	baseDir  string
	delegate FileSystemProvider
}

// NewCloudFSProvider creates a provider that prefixes the tenant's organization ID
// to all paths, effectively chrooting file access per tenant before delegating
// the actual file operation to the underlying filesystem provider.
func NewCloudFSProvider(baseDir string, delegate FileSystemProvider) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir:  baseDir,
		delegate: delegate,
	}
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, path string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", errors.New("unauthorized: no organization ID in context")
	}

	// The delegate (LocalFSProvider) already prepends baseDir.
	// So we just return orgID/path.
	tenantPath := filepath.Join(orgID, path)
	return tenantPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return p.delegate.ReadFile(ctx, tenantPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	return p.delegate.WriteFile(ctx, tenantPath, data)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return p.delegate.ListDir(ctx, tenantPath)
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	// Delegate down if the underlying provider supports SearchFiles
	// Since we know our LocalFSProvider does, we can cast and call it.
	if searcher, ok := p.delegate.(interface {
		SearchFiles(ctx context.Context, path, pattern string) ([]string, error)
	}); ok {
		return searcher.SearchFiles(ctx, tenantPath, pattern)
	}

	return nil, fmt.Errorf("underlying provider does not support SearchFiles")
}

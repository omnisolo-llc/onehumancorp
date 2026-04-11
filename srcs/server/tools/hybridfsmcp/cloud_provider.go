package hybridfsmcp

import (
	"context"
	"errors"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider using an underlying provider
// and injecting tenant isolation via auth.Claims.
type CloudFSProvider struct {
	delegate FileSystemProvider
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(delegate FileSystemProvider) *CloudFSProvider {
	return &CloudFSProvider{
		delegate: delegate,
	}
}

// resolvePath scopes the path to the tenant's organization ID.
func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant claims")
	}

	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	// Ensure we don't prepend if it already starts with it
	if strings.HasPrefix(cleanPath, claims.OrganizationID+"/") {
		return cleanPath, nil
	}

	if cleanPath == claims.OrganizationID {
		return cleanPath, nil
	}

	return filepath.Join(claims.OrganizationID, cleanPath), nil
}

// ReadFile reads the content of a file scoped to the tenant.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	scopedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return p.delegate.ReadFile(ctx, scopedPath)
}

// WriteFile writes content to a file scoped to the tenant.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	scopedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	return p.delegate.WriteFile(ctx, scopedPath, content)
}

// ListDir lists files and directories scoped to the tenant.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]map[string]interface{}, error) {
	scopedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return p.delegate.ListDir(ctx, scopedPath)
}

// IsLocal returns false for cloud providers.
func (p *CloudFSProvider) IsLocal() bool {
	return false
}

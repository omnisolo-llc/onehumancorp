package hybridfsmcp

import (
	"context"
	"errors"

	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider with tenant isolation.
// For the purpose of this implementation, it wraps LocalFSProvider, but prefixes
// all paths with the tenant's OrganizationID. In a real system, this could
// map to S3 or a chrooted NFS mount.
type CloudFSProvider struct {
	delegate *LocalFSProvider
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(basePath string) (*CloudFSProvider, error) {
	delegate, err := NewLocalFSProvider(basePath)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{delegate: delegate}, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// resolveTenantPath prefixes the requested path with the tenant's OrganizationID.
func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or OrganizationID")
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

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	tenantPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	matches, err := p.delegate.SearchFiles(ctx, tenantPath, pattern)
	if err != nil {
		return nil, err
	}

	// The delegate returns paths relative to its basePath (which includes the tenant prefix).
	// We should strip the tenant prefix from the returned paths to make it transparent to the user.
	claims := auth.ClaimsFromContext(ctx)
	var strippedMatches []string
	prefix := claims.OrganizationID + "/"
	for _, match := range matches {
		if strings.HasPrefix(match, prefix) {
			strippedMatches = append(strippedMatches, strings.TrimPrefix(match, prefix))
		} else {
			// fallback for edge cases
			strippedMatches = append(strippedMatches, match)
		}
	}

	return strippedMatches, nil
}

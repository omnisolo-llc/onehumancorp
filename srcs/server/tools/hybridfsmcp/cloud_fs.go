package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for Cloud mode,
// bounding access to a tenant-specific virtual directory derived from context.
type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider(basePath string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{basePath: absBase}, nil
}

// resolveTenantPath determines the tenant's bounded workspace.
func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("access denied: missing or invalid tenant claims")
	}

	tenantWorkspace := filepath.Join(p.basePath, claims.OrganizationID)
	fullPath := filepath.Join(tenantWorkspace, target)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != tenantWorkspace && !strings.HasPrefix(cleanPath, tenantWorkspace+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes tenant boundary")
	}

	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	securePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(securePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	securePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(securePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(securePath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	securePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(securePath)
	if err != nil {
		return nil, err
	}

	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

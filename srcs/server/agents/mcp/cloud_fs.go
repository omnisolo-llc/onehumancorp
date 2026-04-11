package mcp

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	baseCloudDir string
	localFS      *LocalFSProvider // We reuse the local FS logic but scoped per tenant
}

func NewCloudFSProvider(baseCloudDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseCloudDir)
	if err != nil {
		return nil, fmt.Errorf("invalid base cloud directory: %w", err)
	}

	// Create a dummy local FS provider, its baseDir will be overridden per request
	local, err := NewLocalFSProvider(absBase)
	if err != nil {
		return nil, err
	}

	return &CloudFSProvider{
		baseCloudDir: absBase,
		localFS:      local,
	}, nil
}

func (p *CloudFSProvider) getTenantProvider(ctx context.Context) (*LocalFSProvider, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing organization ID in context")
	}

	tenantDir := filepath.Join(p.baseCloudDir, fmt.Sprintf("tenant-%s", claims.OrganizationID))
	return NewLocalFSProvider(tenantDir)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return tenantProvider.ReadFile(ctx, path)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return err
	}
	return tenantProvider.WriteFile(ctx, path, content)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return tenantProvider.ListDir(ctx, path)
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	tenantProvider, err := p.getTenantProvider(ctx)
	if err != nil {
		return nil, err
	}
	return tenantProvider.SearchFiles(ctx, path, pattern)
}

package mcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider maps paths to a tenant-scoped Kubernetes Persistent Volume.
type CloudFSProvider struct {
	basePvcDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(basePvcDir string) *CloudFSProvider {
	return &CloudFSProvider{
		basePvcDir: basePvcDir,
	}
}

var tenantIDRegex = regexp.MustCompile(`^[a-zA-Z0-9_-]+$`)

// resolveTenantPath enforces tenant isolation by prefixing the path with the organization ID.
func (p *CloudFSProvider) resolveTenantPath(claims *auth.Claims, reqPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("missing organization ID in claims")
	}

	if !tenantIDRegex.MatchString(claims.OrganizationID) {
		return "", fmt.Errorf("invalid organization ID format")
	}

	tenantBase := filepath.Join(p.basePvcDir, claims.OrganizationID)

	absBase, err := filepath.Abs(tenantBase)
	if err != nil {
		return "", fmt.Errorf("invalid base directory: %w", err)
	}

	absReq, err := filepath.Abs(filepath.Join(absBase, reqPath))
	if err != nil {
		return "", fmt.Errorf("invalid request path: %w", err)
	}

	rel, err := filepath.Rel(absBase, absReq)
	if err != nil {
		return "", fmt.Errorf("path is outside tenant directory")
	}

	if rel == ".." || filepath.HasPrefix(filepath.ToSlash(rel), "../") {
		return "", fmt.Errorf("path traversal attempt detected")
	}

	return absReq, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.resolveTenantPath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := p.resolveTenantPath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	safePath, err := p.resolveTenantPath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

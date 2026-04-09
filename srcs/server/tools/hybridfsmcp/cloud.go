package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider maps to Tenant-scoped paths, using auth.Claims to scope paths.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{
		baseDir: absBase,
	}, nil
}

func (c *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {

		return "", fmt.Errorf("unauthorized: missing or invalid tenant claims")
	}
	return filepath.Join(c.baseDir, claims.OrganizationID), nil
}

func (c *CloudFSProvider) securePath(ctx context.Context, reqPath string) (string, error) {
	tenantDir, err := c.getTenantDir(ctx)
	if err != nil {
		return "", err
	}

	targetPath := filepath.Join(tenantDir, reqPath)
	var evalErr error
	targetPath, evalErr = filepath.EvalSymlinks(targetPath)
	if evalErr != nil {
		targetPath = filepath.Join(tenantDir, reqPath)
	}
	absPath, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	cleanTenantDir := filepath.Clean(tenantDir)
	if absPath != cleanTenantDir && !strings.HasPrefix(absPath, cleanTenantDir+string(filepath.Separator)) {
		return "", fmt.Errorf("access denied: path escapes tenant directory")
	}
	return absPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := c.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := c.securePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := c.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

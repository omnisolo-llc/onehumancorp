package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	baseVolumeDir string
}

func NewCloudFSProvider(baseVolumeDir string) *CloudFSProvider {
	return &CloudFSProvider{baseVolumeDir: filepath.Clean(baseVolumeDir)}
}

func (c *CloudFSProvider) securePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}

	tenantDir := filepath.Join(c.baseVolumeDir, claims.OrganizationID)
	fullPath := filepath.Clean(filepath.Join(tenantDir, path))

	if !strings.HasPrefix(fullPath, tenantDir+string(os.PathSeparator)) && fullPath != tenantDir {
		return "", fmt.Errorf("path traversal attempt: %s", path)
	}
	return fullPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := c.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := c.securePath(ctx, path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := c.securePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var res []string
	for _, entry := range entries {
		res = append(res, entry.Name())
	}
	return res, nil
}

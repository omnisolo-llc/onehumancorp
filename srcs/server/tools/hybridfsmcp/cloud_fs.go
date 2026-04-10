package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode with tenant isolation.
type CloudFSProvider struct {
	CloudBaseDir string
}

func NewCloudFSProvider(cloudBaseDir string) *CloudFSProvider {
	return &CloudFSProvider{CloudBaseDir: filepath.Clean(cloudBaseDir)}
}

func (c *CloudFSProvider) resolveTenantPath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	if filepath.IsAbs(targetPath) {
		return "", errors.New("absolute paths are not allowed")
	}

	tenantDir := filepath.Join(c.CloudBaseDir, claims.OrganizationID)
	cleanTarget := filepath.Clean(targetPath)
	fullPath := filepath.Join(tenantDir, cleanTarget)

	if !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) && fullPath != tenantDir {
		return "", errors.New("path traversal detected")
	}
	return fullPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := c.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := c.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := c.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

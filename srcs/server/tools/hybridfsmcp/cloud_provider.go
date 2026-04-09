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

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (c *CloudFSProvider) resolveAndValidatePath(claims *auth.Claims, requestedPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("access denied: missing tenant context")
	}

	tenantDir := filepath.Join(c.baseDir, claims.OrganizationID)
	absPath := filepath.Clean(filepath.Join(tenantDir, requestedPath))

	// Ensure the path is within the tenant directory
	if !strings.HasPrefix(absPath, tenantDir+string(filepath.Separator)) && absPath != tenantDir {
		return "", errors.New("access denied: path traversal attempt")
	}

	return absPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	validPath, err := c.resolveAndValidatePath(claims, path)
	if err != nil {
		return nil, err
	}

	return os.ReadFile(validPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	validPath, err := c.resolveAndValidatePath(claims, path)
	if err != nil {
		return err
	}

	// Ensure the directory exists
	if err := os.MkdirAll(filepath.Dir(validPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(validPath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	validPath, err := c.resolveAndValidatePath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(validPath)
	if err != nil {
		return nil, err
	}

	var fileInfos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		fileInfos = append(fileInfos, info)
	}

	return fileInfos, nil
}

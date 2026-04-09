package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for Cloud mode, ensuring tenant isolation.
// For now, it maps to a persistent volume root and scopes paths using auth.Claims.OrganizationID.
type CloudFSProvider struct {
	VolumeRoot string
}

func NewCloudFSProvider(volumeRoot string) (*CloudFSProvider, error) {
	absVolume, err := filepath.Abs(volumeRoot)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{VolumeRoot: absVolume}, nil
}

func (c *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}
	return claims.OrganizationID, nil
}

func (c *CloudFSProvider) sanitizePath(ctx context.Context, path string) (string, error) {
	tenantID, err := c.getTenantDir(ctx)
	if err != nil {
		return "", err
	}

	cleanedPath := filepath.Clean(path)
	if filepath.IsAbs(cleanedPath) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	// tenant root is VolumeRoot/tenantID
	tenantRoot := filepath.Join(c.VolumeRoot, tenantID)

	fullPath := filepath.Join(tenantRoot, cleanedPath)

	// Ensure we don't traverse out of the tenant's root
	if !strings.HasPrefix(fullPath, tenantRoot+string(filepath.Separator)) && fullPath != tenantRoot {
		return "", fmt.Errorf("path access denied")
	}

	return fullPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := c.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := c.sanitizePath(ctx, path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(safePath), 0700); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0600)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := c.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (c *CloudFSProvider) SearchFiles(ctx context.Context, directory string, pattern string) ([]string, error) {
	tenantID, err := c.getTenantDir(ctx)
	if err != nil {
		return nil, err
	}
	tenantRoot := filepath.Join(c.VolumeRoot, tenantID)

	safeDir, err := c.sanitizePath(ctx, directory)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.Walk(safeDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			if matched, _ := filepath.Match(pattern, info.Name()); matched {
				relPath, _ := filepath.Rel(tenantRoot, path)
				matches = append(matches, relPath)
			}
		}
		return nil
	})

	return matches, err
}

package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	mountPoint string
}

func NewCloudFSProvider(mountPoint string) *CloudFSProvider {
	absPath, _ := filepath.Abs(mountPoint)
	return &CloudFSProvider{
		mountPoint: absPath,
	}
}

// resolvePath ensures the path is bounded within the tenant's specific directory.
func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if p.mountPoint == "" {
		return "", errors.New("cloud mount point not configured")
	}

	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant organization ID in claims")
	}

	// Tenant-specific base directory: <mountPoint>/tenant_<orgID>
	tenantBaseDir := filepath.Join(p.mountPoint, "tenant_"+claims.OrganizationID)

	joined := filepath.Join(tenantBaseDir, path)
	absPath, err := filepath.Abs(joined)
	if err != nil {
		return "", err
	}

	cleanTenantBase := filepath.Clean(tenantBaseDir)
	cleanTarget := filepath.Clean(absPath)

	if !strings.HasPrefix(cleanTarget, cleanTenantBase+string(filepath.Separator)) && cleanTarget != cleanTenantBase {
		return "", errors.New("path traversal detected: path escapes tenant directory")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		// If dir doesn't exist, just return empty list
		if os.IsNotExist(err) {
			return []FileInfo{}, nil
		}
		return nil, err
	}

	var results []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries we can't get info for
		}
		results = append(results, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return results, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}

	var results []string

	// If base path doesn't exist, just return empty
	if _, err := os.Stat(resolvedPath); os.IsNotExist(err) {
		return results, nil
	}

	tenantBaseDir := filepath.Join(p.mountPoint, "tenant_"+claims.OrganizationID)

	err = filepath.WalkDir(resolvedPath, func(walkPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil // Skip errors during walk
		}
		if d.IsDir() {
			return nil
		}

		match, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return nil
		}

		if match || strings.Contains(d.Name(), pattern) {
			// Return relative path
			relPath, err := filepath.Rel(tenantBaseDir, walkPath)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})

	return results, err
}

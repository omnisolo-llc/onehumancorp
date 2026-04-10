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

// CloudFSProvider implements FileSystemProvider for a multi-tenant cloud environment,
// scoping file operations to the OrganizationID found in the auth.Claims.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		absBase = baseDir
	}
	return &CloudFSProvider{baseDir: filepath.Clean(absBase)}
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or OrganizationID")
	}

	// Scope to tenant ID inside the base directory
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, path)
	cleanPath := filepath.Clean(fullPath)

	// Prevent path traversal outside the tenant directory
	if !strings.HasPrefix(cleanPath, tenantDir+string(os.PathSeparator)) && cleanPath != tenantDir {
		return "", errors.New("access denied: path escapes tenant boundary")
	}

	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create tenant directory: %w", err)
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			return []fs.FileInfo{}, nil // Return empty list if tenant dir doesn't exist yet
		}
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

func (p *CloudFSProvider) SearchFiles(ctx context.Context, directory string, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(ctx, directory)
	if err != nil {
		return nil, err
	}

	// Calculate the tenant directory for resolving relative paths
	claims := auth.ClaimsFromContext(ctx)
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)

	var results []string
	err = filepath.Walk(resolved, func(path string, info fs.FileInfo, err error) error {
		if err != nil {
			if errors.Is(err, os.ErrNotExist) && path == resolved {
				return nil // tenant dir might not exist yet
			}
			return nil
		}

		if !info.IsDir() {
			matched, err := filepath.Match(pattern, info.Name())
			if err == nil && matched {
				relPath, err := filepath.Rel(tenantDir, path)
				if err == nil {
					results = append(results, relPath)
				}
			}
		}
		return nil
	})

	return results, err
}

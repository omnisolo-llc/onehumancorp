package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for Cloud-native mode.
// It scopes access to a tenant-specific directory using auth.Claims.
type CloudFSProvider struct {
	baseDir string // e.g. /mnt/k8s/tenant-volumes
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	cleanBase := filepath.Clean(baseDir)
	if !filepath.IsAbs(cleanBase) {
		return nil, fmt.Errorf("base directory must be absolute")
	}
	return &CloudFSProvider{baseDir: cleanBase}, nil
}

func (p *CloudFSProvider) getTenantBase(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID in context")
	}

	// Create a safe tenant sub-directory path using OrganizationID as the tenant identifier
	tenantSafe := filepath.Clean(claims.OrganizationID)
	if strings.Contains(tenantSafe, string(filepath.Separator)) || tenantSafe == "." || tenantSafe == ".." {
		return "", fmt.Errorf("invalid tenant ID")
	}

	return filepath.Join(p.baseDir, tenantSafe), nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	tenantBase, err := p.getTenantBase(ctx)
	if err != nil {
		return "", err
	}

	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("path must be relative")
	}

	fullPath := filepath.Join(tenantBase, cleanPath)

	basePrefix := tenantBase
	if !strings.HasSuffix(basePrefix, string(filepath.Separator)) {
		basePrefix += string(filepath.Separator)
	}

	if !strings.HasPrefix(fullPath, basePrefix) && fullPath != tenantBase {
		return "", fmt.Errorf("path escapes tenant directory")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) (string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return "", err
	}

	data, err := os.ReadFile(fullPath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content string) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, []byte(content), 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

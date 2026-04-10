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

type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider(basePath string) *CloudFSProvider {
	return &CloudFSProvider{basePath: basePath}
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", fmt.Errorf("unauthorized: claims not found")
	}

	tenantID := claims.OrganizationID
	if tenantID == "" {
		return "", fmt.Errorf("tenant ID not found in claims")
	}

	cleanPath := filepath.Clean(path)
	tenantBasePath := filepath.Join(p.basePath, tenantID)
	fullPath := filepath.Join(tenantBasePath, cleanPath)

	base := filepath.Clean(tenantBasePath)
	if !strings.HasSuffix(base, string(os.PathSeparator)) {
		base += string(os.PathSeparator)
	}

	checkPath := fullPath
	if !strings.HasSuffix(checkPath, string(os.PathSeparator)) {
		checkPath += string(os.PathSeparator)
	}

	if !strings.HasPrefix(checkPath, base) && fullPath != filepath.Clean(tenantBasePath) {
		return "", fmt.Errorf("path escapes tenant directory: %s", path)
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	infos := make([]fs.FileInfo, 0, len(entries))
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}

	return infos, nil
}

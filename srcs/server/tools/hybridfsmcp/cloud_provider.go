package hybridfsmcp

import (
	"context"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	// In a real cloud setup this would talk to S3 or a k8s PVC scoped by tenant.
	// For testing and mockup, we'll use an in-memory or pseudo-cloud map.
	// We'll simulate tenant-scoping here.
	mockData map[string][]byte
}

func NewCloudFSProvider() *CloudFSProvider {
	return &CloudFSProvider{
		mockData: make(map[string][]byte),
	}
}

func (p *CloudFSProvider) getTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}
	// Simple path sanitization
	if strings.Contains(path, "..") {
		return "", fmt.Errorf("invalid path")
	}
	return fmt.Sprintf("%s/%s", claims.OrganizationID, path), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	data, ok := p.mockData[tenantPath]
	if !ok {
		return nil, fmt.Errorf("file not found: %s", path)
	}
	return data, nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	tenantPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return err
	}
	p.mockData[tenantPath] = append([]byte(nil), data...)
	return nil
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	tenantPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	var res []FileInfo
	prefix := tenantPath
	if !strings.HasSuffix(prefix, "/") && path != "" {
		prefix += "/"
	}

	for k, v := range p.mockData {
		if strings.HasPrefix(k, prefix) || (path == "" && strings.HasPrefix(k, tenantPath+"/")) {
			// simplified: just return the remaining part as file
			relPath := strings.TrimPrefix(k, prefix)
			// check if it's a direct child (no more slashes)
			if !strings.Contains(relPath, "/") {
				res = append(res, FileInfo{
					Name:  relPath,
					IsDir: false,
					Size:  int64(len(v)),
				})
			}
		}
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]FileInfo, error) {
	tenantPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	var res []FileInfo
	prefix := tenantPath
	if !strings.HasSuffix(prefix, "/") && path != "" {
		prefix += "/"
	}

	for k, v := range p.mockData {
		if strings.HasPrefix(k, prefix) || (path == "" && strings.HasPrefix(k, tenantPath+"/")) {
			relPath := strings.TrimPrefix(k, prefix)
			if strings.Contains(relPath, pattern) {
				res = append(res, FileInfo{
					Name:  relPath,
					IsDir: false, // simplified
					Size:  int64(len(v)),
				})
			}
		}
	}
	return res, nil
}

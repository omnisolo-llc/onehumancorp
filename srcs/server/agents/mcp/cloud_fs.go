package mcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for a multi-tenant cloud environment.
// For now, it simulates cloud storage by wrapping a local directory but enforcing tenant isolation.
type CloudFSProvider struct {
	basePath string
}

// NewCloudFSProvider creates a new CloudFSProvider with the given base directory for persistent volumes.
func NewCloudFSProvider(basePath string) (*CloudFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(absPath, 0755); err != nil {
		return nil, err
	}
	return &CloudFSProvider{basePath: absPath}, nil
}

func (p *CloudFSProvider) getScopedPath(claims *auth.Claims, reqPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	cleanPath := filepath.Clean("/" + reqPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	// Create a tenant-specific base path
	tenantBasePath := filepath.Join(p.basePath, claims.OrganizationID)
	fullPath := filepath.Join(tenantBasePath, cleanPath)

	// Ensure no directory traversal escapes tenantBasePath
	if !strings.HasPrefix(fullPath, tenantBasePath) {
		return "", fmt.Errorf("invalid path: %s", reqPath)
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.getScopedPath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.getScopedPath(claims, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error) {
	fullPath, err := p.getScopedPath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		size := int64(0)
		if err == nil && !entry.IsDir() {
			size = info.Size()
		}

		results = append(results, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  size,
		})
	}

	return results, nil
}

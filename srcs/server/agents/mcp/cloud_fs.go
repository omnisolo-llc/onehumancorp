package mcp

import (
	"context"
	"errors"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for the cloud, isolating by tenant.
type CloudFSProvider struct {
	baseStoragePath string
}

// NewCloudFSProvider creates a new CloudFSProvider backed by baseStoragePath.
func NewCloudFSProvider(baseStoragePath string) (*CloudFSProvider, error) {
	absPath, err := filepath.Abs(baseStoragePath)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseStoragePath: absPath}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid tenant claims")
	}

	tenantID := claims.OrganizationID

	// Clean and join the requested path with the base path + tenant ID
	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		cleanPath = filepath.Clean(strings.TrimPrefix(cleanPath, "/"))
	}

	tenantBasePath := filepath.Join(p.baseStoragePath, tenantID)
	fullPath := filepath.Join(tenantBasePath, cleanPath)

	rel, err := filepath.Rel(tenantBasePath, fullPath)
	if err != nil {
		return "", errors.New("access denied: cannot resolve path")
	}

	if rel == ".." || strings.HasPrefix(rel, "../") {
		return "", errors.New("access denied: path outside tenant workspace")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	f, err := os.Open(fullPath)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	return io.ReadAll(f)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var files []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		files = append(files, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}

	return files, nil
}

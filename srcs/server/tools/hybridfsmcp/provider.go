package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
	return &LocalFSProvider{basePath: basePath}
}

func (l *LocalFSProvider) resolvePath(p string) (string, error) {
	if strings.Contains(p, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	cleaned := filepath.Clean("/" + p)
	cleaned = strings.TrimPrefix(cleaned, "/")
	fullPath := filepath.Join(l.basePath, cleaned)
	return fullPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider(basePath string) *CloudFSProvider {
	return &CloudFSProvider{basePath: basePath}
}

func (c *CloudFSProvider) getTenantPath(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: claims not found in context")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization id")
	}
	return filepath.Join(c.basePath, claims.OrganizationID), nil
}

func (c *CloudFSProvider) resolvePath(ctx context.Context, p string) (string, error) {
	tenantPath, err := c.getTenantPath(ctx)
	if err != nil {
		return "", err
	}
	if strings.Contains(p, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	cleaned := filepath.Clean("/" + p)
	cleaned = strings.TrimPrefix(cleaned, "/")
	fullPath := filepath.Join(tenantPath, cleaned)
	return fullPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

package mcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for File System operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for the local standalone mode.
type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: baseDir}
}

func (l *LocalFSProvider) sanitizePath(path string) (string, error) {
	if strings.Contains(path, "..") {
		return "", errors.New("invalid path: contains '..'")
	}
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	fullPath := filepath.Join(l.BaseDir, cleanPath)
	return fullPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := l.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := l.sanitizePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := l.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (l *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for the cloud-native multi-tenant mode.
// In reality, this would map to a tenant-scoped PVC or S3 bucket.
// For demonstration, we'll map it to a local BaseDir but heavily enforce claims.
type CloudFSProvider struct {
	BaseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseDir: baseDir}
}

func (c *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}
	if strings.Contains(path, "..") {
		return "", errors.New("invalid path: contains '..'")
	}
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	// Namespace by OrganizationID
	tenantPath := filepath.Join(c.BaseDir, claims.OrganizationID, cleanPath)
	return tenantPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := c.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := c.resolvePath(claims, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := c.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return []string{}, nil // return empty if dir doesn't exist
		}
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (c *CloudFSProvider) IsLocal() bool {
	return false
}

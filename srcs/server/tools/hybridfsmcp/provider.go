package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded by baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
	if strings.Contains(targetPath, "..") {
		return "", errors.New("directory traversal not allowed")
	}

	cleanPath := filepath.Clean("/" + targetPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	fullPath := filepath.Join(p.baseDir, cleanPath)
	if !strings.HasPrefix(fullPath, filepath.Clean(p.baseDir)) {
		return "", errors.New("path escapes base directory")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	secure, err := p.securePath(path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}

	return os.WriteFile(secure, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		results = append(results, entry.Name())
	}
	return results, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant scoping.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider backed by a base storage directory.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) securePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid tenant claims")
	}

	if strings.Contains(targetPath, "..") {
		return "", errors.New("directory traversal not allowed")
	}

	cleanPath := filepath.Clean("/" + targetPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanPath)

	if !strings.HasPrefix(fullPath, filepath.Clean(tenantDir)) {
		return "", errors.New("path escapes tenant directory")
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}

	return os.WriteFile(secure, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		results = append(results, entry.Name())
	}
	return results, nil
}

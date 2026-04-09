package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for a local directory.
type LocalFSProvider struct {
	baseDir string
}

// CloudFSProvider implements FileSystemProvider for tenant-isolated cloud paths.
type CloudFSProvider struct {
	baseDir string
}

// NewFileSystemProvider creates a FileSystemProvider based on environment.
func NewFileSystemProvider(isLocal bool, baseDir string) FileSystemProvider {
	if isLocal {
		return &LocalFSProvider{baseDir: baseDir}
	}
	return &CloudFSProvider{baseDir: baseDir}
}

// safeLocalPath ensures that the resolved path does not escape the base directory.
func (p *LocalFSProvider) safeLocalPath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	fullPath := filepath.Join(p.baseDir, cleanTarget)

	// Ensure we don't traverse outside of baseDir
	rel, err := filepath.Rel(p.baseDir, fullPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}

// ReadFile reads a file from the local file system.
func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.safeLocalPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

// WriteFile writes a file to the local file system.
func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	safePath, err := p.safeLocalPath(path)
	if err != nil {
		return err
	}

	// Create directory if it doesn't exist
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, content, 0644)
}

// ListDir lists the contents of a local directory.
func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := p.safeLocalPath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}


// safeCloudPath scopes a path to a tenant ID.
func (p *CloudFSProvider) safeCloudPath(claims *auth.Claims, target string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	cleanTarget := filepath.Clean(target)
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanTarget)

	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}


// ReadFile reads a file from the cloud/tenant-isolated file system.
func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.safeCloudPath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

// WriteFile writes a file to the cloud/tenant-isolated file system.
func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	safePath, err := p.safeCloudPath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, content, 0644)
}

// ListDir lists the contents of a cloud/tenant-isolated directory.
func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := p.safeCloudPath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

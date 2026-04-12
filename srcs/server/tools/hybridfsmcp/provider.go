package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.workspaceDir, reqPath))
	if !strings.HasPrefix(fullPath, p.workspaceDir+string(filepath.Separator)) && fullPath != p.workspaceDir {
		return "", fmt.Errorf("path traversal attempt or access denied")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure parent directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	var matches []string
	err := filepath.WalkDir(p.workspaceDir, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() && strings.Contains(d.Name(), query) {
			rel, _ := filepath.Rel(p.workspaceDir, path)
			matches = append(matches, rel)
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, simulating tenant-scoped access.
// In a real K8s environment, this would map to a PVC or S3. Here we simulate it with tenant-scoped local dirs for now.
type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{baseStorageDir: filepath.Clean(baseStorageDir)}
}

func (p *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}
	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)
	return filepath.Clean(tenantDir), nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return "", err
	}
	fullPath := filepath.Clean(filepath.Join(tenantDir, reqPath))
	if !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) && fullPath != tenantDir {
		return "", fmt.Errorf("path traversal attempt or access denied")
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(tenantDir, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() && strings.Contains(d.Name(), query) {
			rel, _ := filepath.Rel(tenantDir, path)
			matches = append(matches, rel)
		}
		return nil
	})
	return matches, err
}

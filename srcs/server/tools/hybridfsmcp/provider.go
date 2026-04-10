package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for our hybrid file system.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, path string, pattern string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a LocalFSProvider with the given base path.
func NewLocalFSProvider(basePath string) *LocalFSProvider {
	return &LocalFSProvider{basePath: filepath.Clean(basePath)}
}

// validatePath checks if the target path is safely within the basePath.
func (p *LocalFSProvider) validatePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.basePath, target))
	if cleanTarget == p.basePath || strings.HasPrefix(cleanTarget, p.basePath+string(filepath.Separator)) {
		return cleanTarget, nil
	}
	return "", errors.New("path traversal detected")
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	safePath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	// Ensure parent directories exist
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, entry := range entries {
		res = append(res, entry.Name())
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(safePath, func(currPath string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(p.basePath, currPath)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode.
type CloudFSProvider struct {
	basePath string
}

// NewCloudFSProvider creates a CloudFSProvider with the given base path.
func NewCloudFSProvider(basePath string) *CloudFSProvider {
	return &CloudFSProvider{basePath: filepath.Clean(basePath)}
}

// getTenantBasePath retrieves the tenant-scoped base path using auth.Claims.
func (p *CloudFSProvider) getTenantBasePath(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization_id")
	}
	return filepath.Join(p.basePath, claims.OrganizationID), nil
}

// validatePath checks if the target path is safely within the tenant's base path.
func (p *CloudFSProvider) validatePath(ctx context.Context, target string) (string, error) {
	tenantBasePath, err := p.getTenantBasePath(ctx)
	if err != nil {
		return "", err
	}
	cleanTarget := filepath.Clean(filepath.Join(tenantBasePath, target))
	if cleanTarget == tenantBasePath || strings.HasPrefix(cleanTarget, tenantBasePath+string(filepath.Separator)) {
		return cleanTarget, nil
	}
	return "", errors.New("path traversal detected or cross-tenant access attempted")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	safePath, err := p.validatePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, entry := range entries {
		res = append(res, entry.Name())
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	safePath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	tenantBasePath, err := p.getTenantBasePath(ctx)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(safePath, func(currPath string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(tenantBasePath, currPath)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

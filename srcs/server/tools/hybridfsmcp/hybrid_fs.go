package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanPath := filepath.Clean(target)
	if filepath.IsAbs(cleanPath) {
		return "", errors.New("absolute paths are not allowed")
	}

	fullPath := filepath.Join(p.baseDir, cleanPath)

	// Boundary check
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("path escapes base directory")
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
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

	var result []string
	for _, e := range entries {
		result = append(result, e.Name())
	}
	return result, nil
}

// CloudFSProvider implements FileSystemProvider for Multi-tenant mode
type CloudFSProvider struct {
	pvBaseDir string
}

func NewCloudFSProvider(pvBaseDir string) *CloudFSProvider {
	return &CloudFSProvider{pvBaseDir: pvBaseDir}
}

func (p *CloudFSProvider) getTenantBaseDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant context")
	}
	return filepath.Join(p.pvBaseDir, claims.OrganizationID), nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	tenantBase, err := p.getTenantBaseDir(ctx)
	if err != nil {
		return "", err
	}

	cleanPath := filepath.Clean(target)
	if filepath.IsAbs(cleanPath) {
		return "", errors.New("absolute paths are not allowed")
	}

	fullPath := filepath.Join(tenantBase, cleanPath)

	// Boundary check
	if fullPath != tenantBase && !strings.HasPrefix(fullPath, tenantBase+string(filepath.Separator)) {
		return "", errors.New("path escapes tenant directory")
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

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		// If the tenant directory does not exist yet, return empty list instead of error
		if errors.Is(err, fs.ErrNotExist) {
			return []string{}, nil
		}
		return nil, err
	}

	var result []string
	for _, e := range entries {
		result = append(result, e.Name())
	}
	return result, nil
}

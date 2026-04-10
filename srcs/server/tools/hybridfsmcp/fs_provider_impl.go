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

// LocalFSProvider implements FileSystemProvider for Standalone mode
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a new LocalFSProvider
func NewLocalFSProvider(basePath string) *LocalFSProvider {
	return &LocalFSProvider{basePath: basePath}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.basePath, target))

	// Directory bounds check to prevent path traversal
	if cleanTarget != p.basePath && !strings.HasPrefix(cleanTarget, p.basePath+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes base directory")
	}

	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolvedPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}

	return infos, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant scoping
type CloudFSProvider struct {
	mountPath string
}

// NewCloudFSProvider creates a new CloudFSProvider
func NewCloudFSProvider(mountPath string) *CloudFSProvider {
	return &CloudFSProvider{mountPath: mountPath}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid claims")
	}

	tenantBasePath := filepath.Join(p.mountPath, claims.OrganizationID)
	cleanTarget := filepath.Clean(filepath.Join(tenantBasePath, target))

	// Tenant bounds check
	if cleanTarget != tenantBasePath && !strings.HasPrefix(cleanTarget, tenantBasePath+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes tenant directory")
	}

	return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolvedPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}

	return infos, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]os.FileInfo, error)
}

// LocalFSProvider implements file system operations mapped directly to the local FS with safety bounds.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider() *LocalFSProvider {
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = os.TempDir()
	}
	absBase, err := filepath.Abs(baseDir)
	if err == nil {
		baseDir = absBase
	}
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.baseDir, target))
	if err != nil {
		return "", err
	}
	// Prevent path traversal vulnerabilities
	if absTarget == p.baseDir || strings.HasPrefix(absTarget, p.baseDir+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", fmt.Errorf("path traversal attempt blocked: %s", target)
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []os.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

// CloudFSProvider implements file system operations mapped to tenant-scoped virtual directories.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider() *CloudFSProvider {
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = "/mnt/tenant-data" // Default for k8s volumes
	}
	absBase, err := filepath.Abs(baseDir)
	if err == nil {
		baseDir = absBase
	}
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("missing tenant context")
	}

	tenantBase := filepath.Join(p.baseDir, claims.OrganizationID)
	absTarget, err := filepath.Abs(filepath.Join(tenantBase, target))
	if err != nil {
		return "", err
	}

	// Prevent path traversal vulnerabilities outside the tenant base
	if absTarget == tenantBase || strings.HasPrefix(absTarget, tenantBase+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", fmt.Errorf("path traversal attempt blocked: %s", target)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []os.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

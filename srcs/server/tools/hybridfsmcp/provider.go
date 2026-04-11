package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded to a workspace directory
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	// ensure dir exists
	if err := os.MkdirAll(absBase, 0755); err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.baseDir, target))
	if err != nil {
		return "", err
	}

	// Memory rule: To prevent path traversal vulnerabilities when verifying directory boundaries,
	// validate using `target == base || strings.HasPrefix(target, base+string(filepath.Separator))`
	if absTarget == p.baseDir || strings.HasPrefix(absTarget, p.baseDir+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", fmt.Errorf("path traversal attempt: %s", target)
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// ensure parent dir exists
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	infos := make([]fs.FileInfo, 0, len(entries))
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode, bounded to tenant-specific directories
type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseStorageDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseStorageDir: absBase}, nil
}

func (p *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing or invalid auth claims")
	}
	// memory rule: scope paths by organization_id extracted from the context's injected auth.Claims
	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)
	// ensure tenant dir exists
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		return "", err
	}
	return tenantDir, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return "", err
	}

	absTarget, err := filepath.Abs(filepath.Join(tenantDir, target))
	if err != nil {
		return "", err
	}

	if absTarget == tenantDir || strings.HasPrefix(absTarget, tenantDir+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", fmt.Errorf("cross-tenant path traversal attempt: %s", target)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	infos := make([]fs.FileInfo, 0, len(entries))
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}
	return infos, nil
}

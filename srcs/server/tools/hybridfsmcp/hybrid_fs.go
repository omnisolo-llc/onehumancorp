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

// FileSystemProvider defines the unified interface for file operations across environments
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded to a specific workspace directory
type LocalFSProvider struct {
	workspaceRoot string
}

func NewLocalFSProvider(root string) (*LocalFSProvider, error) {
	absRoot, err := filepath.Abs(root)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceRoot: absRoot}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.workspaceRoot, targetPath))
	if err != nil {
		return "", err
	}
	// Path traversal protection
	if !strings.HasPrefix(absPath, p.workspaceRoot+string(filepath.Separator)) && absPath != p.workspaceRoot {
		return "", fmt.Errorf("path traversal attempt: %s", targetPath)
	}
	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
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
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider for Multi-tenant Cloud mode
type CloudFSProvider struct {
	baseStorageRoot string
}

func NewCloudFSProvider(root string) (*CloudFSProvider, error) {
	absRoot, err := filepath.Abs(root)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseStorageRoot: absRoot}, nil
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant organization context")
	}

	tenantRoot := filepath.Join(p.baseStorageRoot, "tenants", claims.OrganizationID)
	absPath, err := filepath.Abs(filepath.Join(tenantRoot, targetPath))
	if err != nil {
		return "", err
	}

	// Path traversal protection bound to the tenant root
	if !strings.HasPrefix(absPath, tenantRoot+string(filepath.Separator)) && absPath != tenantRoot {
		return "", fmt.Errorf("path traversal attempt: %s", targetPath)
	}
	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolvedPath, err := p.resolveTenantPath(ctx, path)
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
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

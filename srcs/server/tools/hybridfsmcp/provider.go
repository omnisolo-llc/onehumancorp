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

// FileSystemProvider defines the interface for unified file system operations.
type FileSystemProvider interface {
	IsLocal() bool
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider for the local filesystem.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a new LocalFSProvider with a restricted base directory.
func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(absPath, 0755); err != nil {
		return nil, err
	}
	return &LocalFSProvider{basePath: absPath}, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) getLocalPath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", target)
	}

	fullPath := filepath.Join(p.basePath, cleanTarget)

	// Prevent directory traversal attacks
	if fullPath != p.basePath && !strings.HasPrefix(fullPath, p.basePath+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes base directory: %s", target)
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.getLocalPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.getLocalPath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.getLocalPath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
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

// CloudFSProvider implements FileSystemProvider simulating a cloud K8s PV.
// It scopes operations per-tenant by prefixing paths with the OrganizationID.
type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider(basePath string) (*CloudFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(absPath, 0755); err != nil {
		return nil, err
	}
	return &CloudFSProvider{basePath: absPath}, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) getTenantPath(ctx context.Context, target string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID in context")
	}

	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", target)
	}

	// Scope by base path AND tenant ID
	tenantBase := filepath.Join(p.basePath, orgID)
	if err := os.MkdirAll(tenantBase, 0755); err != nil {
		return "", err
	}

	fullPath := filepath.Join(tenantBase, cleanTarget)

	// Ensure the full path stays within the tenant's base directory
	if fullPath != tenantBase && !strings.HasPrefix(fullPath, tenantBase+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes tenant directory: %s", target)
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
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

package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the standard interface for file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for a local directory boundary.
type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absBase, err := filepath.Abs(p.BaseDir)
	if err != nil {
		return "", err
	}

	absTarget, err := filepath.Abs(filepath.Join(absBase, target))
	if err != nil {
		return "", err
	}

	if absTarget != absBase && !strings.HasPrefix(absTarget, absBase+string(filepath.Separator)) {
		return "", errors.New("path traversal detected: access outside of base directory")
	}

	return absTarget, nil
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
	// Ensure parent dir exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// CloudFSProvider implements FileSystemProvider with tenant isolation.
type CloudFSProvider struct {
	BaseMountPoint string
}

func NewCloudFSProvider(baseMountPoint string) *CloudFSProvider {
	return &CloudFSProvider{BaseMountPoint: baseMountPoint}
}

func (p *CloudFSProvider) getTenantPath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization claims in context")
	}

	// The tenant's isolated root is BaseMountPoint / OrganizationID
	tenantRoot := filepath.Join(p.BaseMountPoint, claims.OrganizationID)

	absBase, err := filepath.Abs(tenantRoot)
	if err != nil {
		return "", err
	}

	absTarget, err := filepath.Abs(filepath.Join(absBase, target))
	if err != nil {
		return "", err
	}

	if absTarget != absBase && !strings.HasPrefix(absTarget, absBase+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal detected: access outside of tenant directory %s", claims.OrganizationID)
	}

	return absTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

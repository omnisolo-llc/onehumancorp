package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

var (
	ErrAccessDenied = errors.New("access denied: path traversal or unauthorized access")
	ErrNotFound     = errors.New("file or directory not found")
)

// FileSystemProvider defines the unified interface for file operations across Hybrid environments.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]os.DirEntry, error)
}

// LocalFSProvider maps to the local file system with path bounding.
type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider() *LocalFSProvider {
	basePath := os.Getenv("OHC_FS_ROOT")
	if basePath == "" {
		basePath, _ = os.Getwd()
	}
	basePath, _ = filepath.Abs(basePath)
	return &LocalFSProvider{basePath: basePath}
}

// resolvePath ensures the target path is within the allowed base path to prevent traversal attacks.
func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.basePath, target))
	if err != nil {
		return "", err
	}

	// Validation using Memory guideline for path traversal:
	if absPath == p.basePath || strings.HasPrefix(absPath, p.basePath+string(filepath.Separator)) {
		return absPath, nil
	}
	return "", ErrAccessDenied
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
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// CloudFSProvider maps to a tenant-scoped file system.
type CloudFSProvider struct {
	basePath string
}

func NewCloudFSProvider() *CloudFSProvider {
	basePath := os.Getenv("OHC_FS_ROOT")
	if basePath == "" {
		basePath = "/data/tenants"
	}
	basePath, _ = filepath.Abs(basePath)
	return &CloudFSProvider{basePath: basePath}
}

// resolvePath scopes the path to the specific tenant organization ID using auth.Claims.
func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("missing organization_id in context claims: %w", ErrAccessDenied)
	}

	tenantPath := filepath.Join(p.basePath, claims.OrganizationID)
	tenantPath, err := filepath.Abs(tenantPath)
	if err != nil {
		return "", err
	}

	absPath, err := filepath.Abs(filepath.Join(tenantPath, target))
	if err != nil {
		return "", err
	}

	if absPath == tenantPath || strings.HasPrefix(absPath, tenantPath+string(filepath.Separator)) {
		return absPath, nil
	}
	return "", ErrAccessDenied
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

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

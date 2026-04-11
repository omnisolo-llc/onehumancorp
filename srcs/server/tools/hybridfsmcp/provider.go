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

// FileSystemProvider abstracts the file reading and writing logic.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded by a workspace.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider() (*LocalFSProvider, error) {
	root := os.Getenv("OHC_FS_ROOT")
	if root == "" {
		root = os.TempDir()
	}
	absRoot, err := filepath.Abs(root)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path of root: %w", err)
	}
	return &LocalFSProvider{basePath: absRoot}, nil
}

func (p *LocalFSProvider) validatePath(reqPath string) (string, error) {
	if filepath.IsAbs(reqPath) {
		return "", errors.New("absolute paths are not allowed")
	}

	target := filepath.Clean(filepath.Join(p.basePath, reqPath))
	if target != p.basePath && !strings.HasPrefix(target, p.basePath+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}

	return target, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	target, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(target)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	target, err := p.validatePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(target)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(target, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	target, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(target)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, scoped by tenant.
type CloudFSProvider struct {
	basePath string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider() (*CloudFSProvider, error) {
	root := os.Getenv("OHC_FS_ROOT")
	if root == "" {
		root = "/mnt/tenant-data" // Standard default for cloud mode
	}
	absRoot, err := filepath.Abs(root)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path of cloud root: %w", err)
	}
	return &CloudFSProvider{basePath: absRoot}, nil
}

func (p *CloudFSProvider) validatePath(claims *auth.Claims, reqPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	if filepath.IsAbs(reqPath) {
		return "", errors.New("absolute paths are not allowed")
	}

	tenantBase := filepath.Clean(filepath.Join(p.basePath, claims.OrganizationID))

	target := filepath.Clean(filepath.Join(tenantBase, reqPath))
	if target != tenantBase && !strings.HasPrefix(target, tenantBase+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}

	return target, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	target, err := p.validatePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(target)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	target, err := p.validatePath(claims, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(target)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(target, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	target, err := p.validatePath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(target)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

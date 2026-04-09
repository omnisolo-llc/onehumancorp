package hybridfs

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
	ErrAccessDenied = errors.New("access denied: path outside allowed directory")
	ErrUnauthorized = errors.New("unauthorized: missing or invalid claims")
	ErrNotFound     = errors.New("not found: file or directory does not exist")
)

// FileSystemProvider defines the interface for filesystem operations in MCP.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a specific basePath.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to basePath.
func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{basePath: absPath}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.basePath, targetPath))
	if err != nil {
		return "", err
	}

	// Ensure the base directory string ends with a path separator
	baseDir := p.basePath
	if !strings.HasSuffix(baseDir, string(filepath.Separator)) {
		baseDir += string(filepath.Separator)
	}

	if !strings.HasPrefix(absTarget, baseDir) && absTarget != p.basePath {
		return "", ErrAccessDenied
	}
	return absTarget, nil
}

// ReadFile reads a file from the bounded local filesystem.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

// WriteFile writes a file to the bounded local filesystem.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, content, 0644)
}

// ListDir lists contents of a directory in the bounded local filesystem.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, ErrNotFound
		}
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode, scoping access to the TenantID.
type CloudFSProvider struct {
	cloudBasePath string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to cloudBasePath.
func NewCloudFSProvider(cloudBasePath string) (*CloudFSProvider, error) {
	absPath, err := filepath.Abs(cloudBasePath)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{cloudBasePath: absPath}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", ErrUnauthorized
	}

	tenantDir := filepath.Join(p.cloudBasePath, claims.OrganizationID)

	// Create the tenant path implicitly if it doesn't exist
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		return "", fmt.Errorf("failed to create tenant directory: %w", err)
	}

	absTarget, err := filepath.Abs(filepath.Join(tenantDir, targetPath))
	if err != nil {
		return "", err
	}

	// Ensure the tenant directory string ends with a path separator
	if !strings.HasSuffix(tenantDir, string(filepath.Separator)) {
		tenantDir += string(filepath.Separator)
	}

	if !strings.HasPrefix(absTarget, tenantDir) && absTarget != filepath.Clean(filepath.Join(p.cloudBasePath, claims.OrganizationID)) {
		return "", ErrAccessDenied
	}

	return absTarget, nil
}

// ReadFile reads a file scoped to the tenant.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

// WriteFile writes a file scoped to the tenant.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, content, 0644)
}

// ListDir lists contents of a directory scoped to the tenant.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, ErrNotFound
		}
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// NewProvider creates the appropriate FileSystemProvider based on the environment mode.
func NewProvider(isCloud bool, basePath string) (FileSystemProvider, error) {
	if isCloud {
		return NewCloudFSProvider(basePath)
	}
	return NewLocalFSProvider(basePath)
}

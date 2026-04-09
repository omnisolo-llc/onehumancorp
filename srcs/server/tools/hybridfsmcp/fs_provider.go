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

// FileSystemProvider defines the interface for our Hybrid File System.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded to a specific workspace directory.
type LocalFSProvider struct {
	WorkspaceRoot string
}

func NewLocalFSProvider(workspaceRoot string) *LocalFSProvider {
	return &LocalFSProvider{
		WorkspaceRoot: filepath.Clean(workspaceRoot),
	}
}

func (l *LocalFSProvider) validatePath(targetPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(l.WorkspaceRoot, targetPath))
	if err != nil {
		return "", err
	}
	// Prevent directory traversal: make sure the path stays within WorkspaceRoot
	if !strings.HasPrefix(absPath, l.WorkspaceRoot+string(filepath.Separator)) && absPath != l.WorkspaceRoot {
		return "", fmt.Errorf("path escapes workspace root: %s", targetPath)
	}
	return absPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := l.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := l.validatePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	absPath, err := l.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, isolating per tenant based on OrganizationID.
type CloudFSProvider struct {
	StorageRoot string
}

func NewCloudFSProvider(storageRoot string) *CloudFSProvider {
	return &CloudFSProvider{
		StorageRoot: filepath.Clean(storageRoot),
	}
}

func (c *CloudFSProvider) getTenantRoot(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", fmt.Errorf("no auth claims found in context")
	}
	if claims.OrganizationID == "" {
		return "", fmt.Errorf("no organization ID found in claims")
	}
	return filepath.Join(c.StorageRoot, claims.OrganizationID), nil
}

func (c *CloudFSProvider) validatePath(ctx context.Context, targetPath string) (string, error) {
	tenantRoot, err := c.getTenantRoot(ctx)
	if err != nil {
		return "", err
	}
	absPath, err := filepath.Abs(filepath.Join(tenantRoot, targetPath))
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(absPath, tenantRoot+string(filepath.Separator)) && absPath != tenantRoot {
		return "", fmt.Errorf("path escapes tenant root: %s", targetPath)
	}
	return absPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := c.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := c.validatePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	absPath, err := c.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

// NewProviderFactory returns the appropriate provider based on the environment
func NewProviderFactory(storageRoot string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(storageRoot)
	}
	return NewCloudFSProvider(storageRoot)
}

package hybridfsmcp

import (
	"context"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

// Ensure the local implementation maps directly to the local file system with safety bounds.
type LocalFSProvider struct {
	WorkspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{WorkspaceDir: workspaceDir}
}

func (l *LocalFSProvider) sanitizePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(l.WorkspaceDir, path))
	if !strings.HasPrefix(cleanPath, l.WorkspaceDir + string(os.PathSeparator)) && cleanPath != l.WorkspaceDir {
		return "", os.ErrPermission
	}
	return cleanPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	cleanPath, err := l.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(cleanPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	cleanPath, err := l.sanitizePath(path)
	if err != nil {
		return err
	}
	// Create directory if it doesn't exist
	dir := filepath.Dir(cleanPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(cleanPath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	cleanPath, err := l.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	dir, err := os.Open(cleanPath)
	if err != nil {
		return nil, err
	}
	defer dir.Close()
	return dir.Readdir(-1)
}

// Cloud Implementation Maps to Tenant-scoped Kubernetes Persistent Volumes or a virtualized S3-backed file system interface.
type CloudFSProvider struct {
	BaseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseDir: baseDir}
}

func (c *CloudFSProvider) sanitizePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", os.ErrPermission // Unauthorized if no claims
	}

	tenantID := claims.OrganizationID
	if tenantID == "" {
		return "", os.ErrPermission // Unauthorized if no tenant ID
	}

	tenantDir := filepath.Join(c.BaseDir, tenantID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, path))

	if !strings.HasPrefix(cleanPath, tenantDir + string(os.PathSeparator)) && cleanPath != tenantDir {
		return "", os.ErrPermission
	}

	return cleanPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	cleanPath, err := c.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(cleanPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	cleanPath, err := c.sanitizePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(cleanPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(cleanPath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	cleanPath, err := c.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	dir, err := os.Open(cleanPath)
	if err != nil {
		return nil, err
	}
	defer dir.Close()
	return dir.Readdir(-1)
}

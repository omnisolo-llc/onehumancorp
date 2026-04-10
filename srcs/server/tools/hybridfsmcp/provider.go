package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for local and cloud file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// LocalFSProvider implements file operations for the standalone mode.
type LocalFSProvider struct {
	WorkspaceDir string
}

func (l *LocalFSProvider) resolvePath(path string) (string, error) {
	absRoot, err := filepath.Abs(l.WorkspaceDir)
	if err != nil {
		return "", err
	}
	absPath, err := filepath.Abs(filepath.Join(absRoot, path))
	if err != nil {
		return "", err
	}
	cleanAbs := filepath.Clean(absPath)
	cleanRoot := filepath.Clean(absRoot)

	if !strings.HasPrefix(cleanAbs, cleanRoot+string(filepath.Separator)) && cleanAbs != cleanRoot {
		return "", errors.New("path traversal detected")
	}
	return cleanAbs, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := l.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

// CloudFSProvider implements file operations for the cloud mode.
type CloudFSProvider struct {
	BaseVolumePath string
}

func (c *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}
	absRoot, err := filepath.Abs(c.BaseVolumePath)
	if err != nil {
		return "", err
	}
	tenantPath := filepath.Join(absRoot, claims.OrganizationID)
	absPath, err := filepath.Abs(filepath.Join(tenantPath, path))
	if err != nil {
		return "", err
	}
	cleanAbs := filepath.Clean(absPath)
	cleanTenant := filepath.Clean(tenantPath)

	if !strings.HasPrefix(cleanAbs, cleanTenant+string(filepath.Separator)) && cleanAbs != cleanTenant {
		return "", errors.New("path traversal detected")
	}
	return cleanAbs, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := c.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := c.resolvePath(claims, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := c.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

// NewProvider creates a new FileSystemProvider depending on the environment.
func NewProvider(isStandalone bool, rootPath string) FileSystemProvider {
	if isStandalone {
		return &LocalFSProvider{WorkspaceDir: rootPath}
	}
	return &CloudFSProvider{BaseVolumePath: rootPath}
}

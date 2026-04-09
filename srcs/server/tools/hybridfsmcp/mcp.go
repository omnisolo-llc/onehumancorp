package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider interface
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider
type LocalFSProvider struct {
	WorkspaceRoot string
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath := filepath.Join(l.WorkspaceRoot, path)
	if !filepath.IsLocal(path) || filepath.Clean(fullPath) != fullPath {
        return nil, fmt.Errorf("invalid path")
    }
	return os.ReadFile(fullPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath := filepath.Join(l.WorkspaceRoot, path)
    if !filepath.IsLocal(path) || filepath.Clean(fullPath) != fullPath {
        return fmt.Errorf("invalid path")
    }
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath := filepath.Join(l.WorkspaceRoot, path)
    if !filepath.IsLocal(path) || filepath.Clean(fullPath) != fullPath {
        return nil, fmt.Errorf("invalid path")
    }
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	names := []string{}
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider
type CloudFSProvider struct {
	BaseDir string
}

func (c *CloudFSProvider) getTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing claims or org id")
	}

	if !filepath.IsLocal(path) {
		return "", fmt.Errorf("invalid path")
	}

	fullPath := filepath.Join(c.BaseDir, claims.OrganizationID, filepath.Clean(path))
	return fullPath, nil
}


func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := c.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := c.getTenantPath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := c.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	names := []string{}
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

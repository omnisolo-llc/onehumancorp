package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

var (
	ErrAccessDenied = errors.New("access denied: path escapes bounds")
	ErrUnauthorized = errors.New("unauthorized: missing claims")
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, pattern string) ([]string, error)
}

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspace string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspace)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: absWorkspace}, nil
}

func (l *LocalFSProvider) securePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(targetPath)
	absPath := filepath.Join(l.workspaceDir, cleanPath)

	// Ensure the resulting absolute path is within the workspace directory
	if !strings.HasPrefix(absPath+string(filepath.Separator), l.workspaceDir+string(filepath.Separator)) {
		return "", ErrAccessDenied
	}
	return absPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	p, err := l.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(p)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	p, err := l.securePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(p)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(p, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	p, err := l.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(p)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (l *LocalFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	// SearchFiles recursively finds files matching the pattern within the workspace
	var matches []string

	err := filepath.WalkDir(l.workspaceDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		relPath, err := filepath.Rel(l.workspaceDir, path)
		if err != nil {
			return nil
		}

		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}

		if matched && !d.IsDir() {
			matches = append(matches, relPath)
		}
		return nil
	})

	return matches, err
}

type CloudFSProvider struct {
	baseProvider *LocalFSProvider
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	lp, err := NewLocalFSProvider(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseProvider: lp}, nil
}

func (c *CloudFSProvider) scopedPath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", ErrUnauthorized
	}

	// Prepend OrganizationID
	cleanPath := filepath.Clean("/" + targetPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	return filepath.Join(claims.OrganizationID, cleanPath), nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	p, err := c.scopedPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return c.baseProvider.ReadFile(ctx, p)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	p, err := c.scopedPath(ctx, path)
	if err != nil {
		return err
	}
	return c.baseProvider.WriteFile(ctx, p, data)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	p, err := c.scopedPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return c.baseProvider.ListDir(ctx, p)
}

func (c *CloudFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, ErrUnauthorized
	}

	// We want to search files within the tenant's base directory
	tenantDir := filepath.Join(c.baseProvider.workspaceDir, claims.OrganizationID)

	var matches []string

	// If tenant dir doesn't exist, just return empty
	if _, err := os.Stat(tenantDir); os.IsNotExist(err) {
		return matches, nil
	}

	err := filepath.WalkDir(tenantDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		relPath, err := filepath.Rel(tenantDir, path)
		if err != nil {
			return nil
		}

		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}

		if matched && !d.IsDir() {
			matches = append(matches, relPath)
		}
		return nil
	})

	return matches, err
}

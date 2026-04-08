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

// FileSystemProvider abstracts file operations for local and cloud modes.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to workspaceDir.
func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	absDir, err := filepath.Abs(workspaceDir)
	if err != nil {
		absDir = filepath.Clean(workspaceDir) // fallback
	}
	return &LocalFSProvider{workspaceDir: absDir}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Join(p.workspaceDir, path)
	cleanPath := filepath.Clean(fullPath)

	prefix := p.workspaceDir
	if !strings.HasSuffix(prefix, string(filepath.Separator)) {
		prefix += string(filepath.Separator)
	}

	if cleanPath != p.workspaceDir && !strings.HasPrefix(cleanPath, prefix) {
		return "", errors.New("path escapes workspace bounds")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var results []string
	err = filepath.WalkDir(resolvedPath, func(currPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil // skip errors
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(p.workspaceDir, currPath)
			results = append(results, rel)
		}
		return nil
	})
	return results, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with multi-tenancy.
type CloudFSProvider struct {
	baseVolumeDir string
}

// NewCloudFSProvider creates a new CloudFSProvider backed by persistent volumes.
func NewCloudFSProvider(baseVolumeDir string) *CloudFSProvider {
	absDir, err := filepath.Abs(baseVolumeDir)
	if err != nil {
		absDir = filepath.Clean(baseVolumeDir) // fallback
	}
	return &CloudFSProvider{baseVolumeDir: absDir}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant organization ID")
	}

	tenantDir := filepath.Join(p.baseVolumeDir, claims.OrganizationID)
	tenantDir = filepath.Clean(tenantDir)

	fullPath := filepath.Join(tenantDir, path)
	cleanPath := filepath.Clean(fullPath)

	prefix := tenantDir
	if !strings.HasSuffix(prefix, string(filepath.Separator)) {
		prefix += string(filepath.Separator)
	}

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, prefix) {
		return "", errors.New("path escapes tenant bounds")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return []string{}, nil
		}
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}

	tenantDir := filepath.Join(p.baseVolumeDir, claims.OrganizationID)
	tenantDir = filepath.Clean(tenantDir)

	var results []string
	err = filepath.WalkDir(resolvedPath, func(currPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(tenantDir, currPath)
			results = append(results, rel)
		}
		return nil
	})
	return results, err
}

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

// FileSystemProvider defines the interface for hybrid file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error)
}

// ensureSafePath checks if the target path is safely within the base directory.
func ensureSafePath(baseDir, targetPath string) (string, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return "", err
	}
	absTarget, err := filepath.Abs(filepath.Join(baseDir, targetPath))
	if err != nil {
		return "", err
	}

	// Ensure trailing separator for prefix check to prevent partial directory name match
	safeBase := absBase
	if !strings.HasSuffix(safeBase, string(filepath.Separator)) {
		safeBase += string(filepath.Separator)
	}

	// If the target is exactly the base dir, it's safe
	if absTarget == absBase {
		return absTarget, nil
	}

	if !strings.HasPrefix(absTarget, safeBase) {
		return "", errors.New("path traversal violation: target escapes base directory")
	}

	return absTarget, nil
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	WorkspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{WorkspaceDir: workspaceDir}
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := ensureSafePath(p.WorkspaceDir, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := ensureSafePath(p.WorkspaceDir, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := ensureSafePath(p.WorkspaceDir, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	safePath, err := ensureSafePath(p.WorkspaceDir, path)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(safePath, func(currPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(p.WorkspaceDir, currPath)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant scoping.
type CloudFSProvider struct {
	BaseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseStorageDir: baseStorageDir}
}

func (p *CloudFSProvider) getTenantDir(claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}
	// tenant scoped directory: baseStorageDir/tenant/<OrganizationID>/
	tenantDir := filepath.Join(p.BaseStorageDir, "tenant", claims.OrganizationID)
	return tenantDir, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	tenantDir, err := p.getTenantDir(claims)
	if err != nil {
		return nil, err
	}
	safePath, err := ensureSafePath(tenantDir, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	tenantDir, err := p.getTenantDir(claims)
	if err != nil {
		return err
	}
	safePath, err := ensureSafePath(tenantDir, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	tenantDir, err := p.getTenantDir(claims)
	if err != nil {
		return nil, err
	}
	safePath, err := ensureSafePath(tenantDir, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	tenantDir, err := p.getTenantDir(claims)
	if err != nil {
		return nil, err
	}
	safePath, err := ensureSafePath(tenantDir, path)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(safePath, func(currPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(tenantDir, currPath)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

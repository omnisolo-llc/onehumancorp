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

// FileSystemProvider defines the interface for unified filesystem access.
type FileSystemProvider interface {
	// IsLocal returns true if the provider is backed by a local workspace.
	IsLocal() bool
	// ReadFile reads the contents of a file.
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	// WriteFile writes contents to a file.
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	// ListDir lists the contents of a directory.
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error)
	// Walk walks the file tree rooted at root, calling walkFn for each file or directory in the tree, including root.
	Walk(ctx context.Context, claims *auth.Claims, root string, walkFn filepath.WalkFunc) error
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	cleanWorkspace, err := filepath.Abs(filepath.Clean(workspaceDir))
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: cleanWorkspace}, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.workspaceDir, path))
	if cleanPath != p.workspaceDir && !strings.HasPrefix(cleanPath, p.workspaceDir+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}
	return infos, nil
}

func (p *LocalFSProvider) Walk(ctx context.Context, claims *auth.Claims, root string, walkFn filepath.WalkFunc) error {
	resolved, err := p.resolvePath(root)
	if err != nil {
		return err
	}

	return filepath.Walk(resolved, func(path string, info fs.FileInfo, err error) error {
		// Calculate the relative path from the workspace dir
		rel, relErr := filepath.Rel(p.workspaceDir, path)
		if relErr != nil {
			return relErr
		}
		// If it's the workspace dir itself, pass "."
		if rel == "." || rel == "" {
			return walkFn(".", info, err)
		}
		return walkFn(rel, info, err)
	})
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, scoping access to tenant-specific directories on persistent volumes.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
    cleanBase, err := filepath.Abs(filepath.Clean(baseDir))
    if err != nil {
        return nil, err
    }
	return &CloudFSProvider{baseDir: cleanBase}, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	tenantDir := filepath.Clean(filepath.Join(p.baseDir, claims.OrganizationID))

	cleanPath := filepath.Clean(filepath.Join(tenantDir, path))
	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("path traversal detected or tenant isolation violated")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}
	return infos, nil
}

func (p *CloudFSProvider) Walk(ctx context.Context, claims *auth.Claims, root string, walkFn filepath.WalkFunc) error {
	resolved, err := p.resolvePath(claims, root)
	if err != nil {
		return err
	}

	tenantDir := filepath.Clean(filepath.Join(p.baseDir, claims.OrganizationID))

	return filepath.Walk(resolved, func(path string, info fs.FileInfo, err error) error {
		// Calculate the relative path from the tenant dir
		rel, relErr := filepath.Rel(tenantDir, path)
		if relErr != nil {
			return relErr
		}
		// If it's the tenant dir itself, pass "."
		if rel == "." || rel == "" {
			return walkFn(".", info, err)
		}
		return walkFn(rel, info, err)
	})
}

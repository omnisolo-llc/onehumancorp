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

// FileSystemProvider defines operations for interacting with files
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
	SearchFiles(ctx context.Context, path, query string) ([]string, error)
}

// LocalFSProvider operates on local file paths, bounded to a workspace directory
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.workspaceDir, target))

	// Ensure the resolved path is within the workspace directory
	if cleanTarget != p.workspaceDir && !strings.HasPrefix(cleanTarget, p.workspaceDir+string(filepath.Separator)) {
		return "", errors.New("path outside workspace boundary")
	}

	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Create directories if they do not exist
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, rootPath, query string) ([]string, error) {
	resolvedRoot, err := p.resolvePath(rootPath)
	if err != nil {
		return nil, err
	}

	var results []string
	err = filepath.WalkDir(resolvedRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), query) {
			relPath, err := filepath.Rel(p.workspaceDir, path)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})
	return results, err
}

// CloudFSProvider operates on local file paths scoped by tenant claims
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid tenant claims")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanTarget := filepath.Clean(filepath.Join(tenantDir, target))

	// Ensure the resolved path is within the tenant directory
	if cleanTarget != tenantDir && !strings.HasPrefix(cleanTarget, tenantDir+string(filepath.Separator)) {
		return "", errors.New("path outside tenant boundary")
	}

	return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	// Create directories if they do not exist
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolved, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, rootPath, query string) ([]string, error) {
	resolvedRoot, err := p.resolveTenantPath(ctx, rootPath)
	if err != nil {
		return nil, err
	}

	claims := auth.ClaimsFromContext(ctx)
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)

	var results []string
	err = filepath.WalkDir(resolvedRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), query) {
			relPath, err := filepath.Rel(tenantDir, path)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})
	return results, err
}

// NewProvider returns a CloudFSProvider or LocalFSProvider based on OHC_MULTITENANT env var.
// In tests or missing workspace, it can fallback to OHC_FS_ROOT or os.TempDir.
func NewProvider(ctx context.Context, workspace string) FileSystemProvider {
	if workspace == "" {
		workspace = os.Getenv("OHC_FS_ROOT")
		if workspace == "" {
			workspace = os.TempDir()
		}
	}

	if strings.ToLower(os.Getenv("OHC_MULTITENANT")) == "true" {
		return NewCloudFSProvider(workspace)
	}
	return NewLocalFSProvider(workspace)
}

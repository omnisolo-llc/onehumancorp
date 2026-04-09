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

// FileSystemProvider abstracts file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
	SearchFiles(ctx context.Context, path string, pattern string) ([]string, error)
}

// LocalFSProvider maps to the local file system, bounded by a workspace root.
type LocalFSProvider struct {
	workspaceRoot string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(workspaceRoot string) (*LocalFSProvider, error) {
	absRoot, err := filepath.Abs(workspaceRoot)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute workspace root: %w", err)
	}
	return &LocalFSProvider{
		workspaceRoot: filepath.Clean(absRoot),
	}, nil
}

func (p *LocalFSProvider) validatePath(targetPath string) (string, error) {
	cleanTarget := filepath.Clean(targetPath)
	fullPath := filepath.Join(p.workspaceRoot, cleanTarget)
	absPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}

	// Validate path bounding to prevent directory traversal
	if !strings.HasPrefix(absPath, p.workspaceRoot+string(filepath.Separator)) && absPath != p.workspaceRoot {
		return "", fmt.Errorf("path bounds check failed: %s is outside workspace root %s", absPath, p.workspaceRoot)
	}

	return absPath, nil
}

// ReadFile reads a file.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

// WriteFile writes a file.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	absPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory for file %s: %w", absPath, err)
	}
	return os.WriteFile(absPath, content, 0644)
}

// ListDir lists a directory.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	absPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

// SearchFiles searches for files.
func (p *LocalFSProvider) SearchFiles(ctx context.Context, rootPath string, pattern string) ([]string, error) {
	absRootPath, err := p.validatePath(rootPath)
	if err != nil {
		return nil, err
	}

	var results []string
	err = filepath.WalkDir(absRootPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			matched, err := filepath.Match(pattern, d.Name())
			if err != nil {
				return err
			}
			if matched {
				relPath, _ := filepath.Rel(p.workspaceRoot, path)
				results = append(results, filepath.ToSlash(relPath))
			}
		}
		return nil
	})

	return results, err
}

// CloudFSProvider maps to tenant-scoped virtual directory or persistent volumes.
type CloudFSProvider struct {
	baseStorageRoot string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseStorageRoot string) (*CloudFSProvider, error) {
	absRoot, err := filepath.Abs(baseStorageRoot)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute base storage root: %w", err)
	}
	return &CloudFSProvider{
		baseStorageRoot: filepath.Clean(absRoot),
	}, nil
}

func (p *CloudFSProvider) getTenantPath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	cleanTarget := filepath.Clean(targetPath)

	// Create the tenant-specific root
	tenantRoot := filepath.Join(p.baseStorageRoot, claims.OrganizationID)

	fullPath := filepath.Join(tenantRoot, cleanTarget)
	absPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}

	// Validate path bounding to prevent directory traversal
	if !strings.HasPrefix(absPath, tenantRoot+string(filepath.Separator)) && absPath != tenantRoot {
		return "", fmt.Errorf("path bounds check failed: %s is outside tenant root %s", absPath, tenantRoot)
	}

	return absPath, nil
}

// ReadFile reads a file.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

// WriteFile writes a file.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	absPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory for file %s: %w", absPath, err)
	}
	return os.WriteFile(absPath, content, 0644)
}

// ListDir lists a directory.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	absPath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

// SearchFiles searches for files.
func (p *CloudFSProvider) SearchFiles(ctx context.Context, rootPath string, pattern string) ([]string, error) {
	absRootPath, err := p.getTenantPath(ctx, rootPath)
	if err != nil {
		return nil, err
	}

	var results []string

	// Tenant root is needed to calculate relative paths correctly
	claims := auth.ClaimsFromContext(ctx)
	tenantRoot := filepath.Join(p.baseStorageRoot, claims.OrganizationID)

	err = filepath.WalkDir(absRootPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err // Can handle specific errors like PermissionDenied here if needed
		}
		if !d.IsDir() {
			matched, err := filepath.Match(pattern, d.Name())
			if err != nil {
				return err
			}
			if matched {
				relPath, _ := filepath.Rel(tenantRoot, path)
				results = append(results, filepath.ToSlash(relPath))
			}
		}
		return nil
	})

	// If the tenant directory doesn't exist, it's not an error, just no files found.
	if os.IsNotExist(err) {
		return []string{}, nil
	}

	return results, err
}

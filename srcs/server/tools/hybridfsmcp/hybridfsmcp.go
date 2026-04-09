package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]os.DirEntry, error)
	SearchFiles(ctx context.Context, path, pattern string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for standalone mode
type LocalFSProvider struct {
	WorkspaceRoot string
}

func NewLocalFSProvider(workspaceRoot string) *LocalFSProvider {
	return &LocalFSProvider{WorkspaceRoot: filepath.Clean(workspaceRoot)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("absolute paths not allowed")
	}
	absPath := filepath.Join(p.WorkspaceRoot, cleanTarget)
	if !strings.HasPrefix(absPath, p.WorkspaceRoot+string(filepath.Separator)) && absPath != p.WorkspaceRoot {
		return "", fmt.Errorf("path traversal attempt")
	}
	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(absPath, func(currPath string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(p.WorkspaceRoot, currPath)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for cloud-native multi-tenant mode
type CloudFSProvider struct {
	MountRoot string
}

func NewCloudFSProvider(mountRoot string) *CloudFSProvider {
	return &CloudFSProvider{MountRoot: filepath.Clean(mountRoot)}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing or invalid claims")
	}

	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("absolute paths not allowed")
	}

	// Scope to tenant's OrganizationID
	tenantPath := filepath.Join(p.MountRoot, claims.OrganizationID)
	absPath := filepath.Join(tenantPath, cleanTarget)

	if !strings.HasPrefix(absPath, tenantPath+string(filepath.Separator)) && absPath != tenantPath {
		return "", fmt.Errorf("path traversal attempt outside tenant scope")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	claims := auth.ClaimsFromContext(ctx)
	tenantPath := filepath.Join(p.MountRoot, claims.OrganizationID)

	var matches []string
	err = filepath.WalkDir(absPath, func(currPath string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(tenantPath, currPath)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// Factory creates the appropriate provider based on environment
func NewProvider(basePath string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(basePath)
	}
	// Default to Cloud multi-tenant mode
	return NewCloudFSProvider(basePath)
}

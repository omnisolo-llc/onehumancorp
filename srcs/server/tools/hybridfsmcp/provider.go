package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the unified interface for file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]os.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: absWorkspace}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.workspaceDir, target))
	if err != nil {
		return "", err
	}

	if absTarget != p.workspaceDir && !strings.HasPrefix(absTarget, p.workspaceDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal vulnerability detected")
	}

	return absTarget, nil
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
    if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
        return err
    }
	return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode, scoping access to tenant directories.
type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseStorageDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseStorageDir: absBase}, nil
}

func (p *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID in context")
	}
	return filepath.Join(p.baseStorageDir, claims.OrganizationID), nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return "", err
	}

	absTarget, err := filepath.Abs(filepath.Join(tenantDir, target))
	if err != nil {
		return "", err
	}

	if absTarget != tenantDir && !strings.HasPrefix(absTarget, tenantDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal vulnerability detected or cross-tenant access denied")
	}

	return absTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
    if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
        return err
    }
	return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// Factory function
func NewProvider(ctx context.Context) (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
        baseDir := os.Getenv("OHC_FS_ROOT")
        if baseDir == "" {
            baseDir = os.TempDir()
        }
		return NewCloudFSProvider(baseDir)
	}
    baseDir := os.Getenv("OHC_FS_ROOT")
    if baseDir == "" {
        baseDir = os.TempDir()
    }
	return NewLocalFSProvider(baseDir)
}

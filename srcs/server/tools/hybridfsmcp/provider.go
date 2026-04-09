package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"regexp"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider is an abstraction for file reading/writing/listing operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]os.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider bounded by a specific workspace directory.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: workspaceDir}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absWorkspace, err := filepath.Abs(p.workspaceDir)
	if err != nil {
		return "", err
	}

	targetPath := filepath.Join(absWorkspace, target)
	absTarget, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(absWorkspace, absTarget)
	if err != nil {
		return "", err
	}
	if rel == ".." || filepath.HasPrefix(rel, ".." + string(filepath.Separator)) {
		return "", fmt.Errorf("path bounds violated: %s", target)
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// CloudFSProvider implements FileSystemProvider bound by multi-tenant claims.
type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{baseStorageDir: baseStorageDir}
}

// sanitizeTenantID ensures the tenant ID contains only alphanumeric characters, underscores, or dashes to prevent directory traversal.
func sanitizeTenantID(id string) error {
	matched, _ := regexp.MatchString("^[a-zA-Z0-9_-]+$", id)
	if !matched {
		return fmt.Errorf("invalid tenant ID format")
	}
	return nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}

	if err := sanitizeTenantID(claims.OrganizationID); err != nil {
		return "", err
	}

	absBase, err := filepath.Abs(p.baseStorageDir)
	if err != nil {
		return "", err
	}

	tenantBase := filepath.Join(absBase, claims.OrganizationID)

	targetPath := filepath.Join(tenantBase, target)
	absTarget, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(tenantBase, absTarget)
	if err != nil {
		return "", err
	}
	if rel == ".." || filepath.HasPrefix(rel, ".." + string(filepath.Separator)) {
		return "", fmt.Errorf("path bounds violated: %s", target)
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

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// NewProviderFactory returns the appropriate provider based on environment variables.
func NewProviderFactory(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

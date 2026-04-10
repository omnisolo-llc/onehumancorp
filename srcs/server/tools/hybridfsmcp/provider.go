package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts file operations for agents.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, scoped to a base directory.
type LocalFSProvider struct {
	BaseDir string
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absBase, err := filepath.Abs(p.BaseDir)
	if err != nil {
		return "", err
	}
	absTarget, err := filepath.Abs(filepath.Join(absBase, target))
	if err != nil {
		return "", err
	}

	// Prevent path traversal
	if absTarget == absBase || strings.HasPrefix(absTarget, absBase+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", fmt.Errorf("access denied: %s is outside base directory", target)
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, scoping to tenant directory.
type CloudFSProvider struct {
	BaseDir string
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant information")
	}

	tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)
	absTenant, err := filepath.Abs(tenantDir)
	if err != nil {
		return "", err
	}

	absTarget, err := filepath.Abs(filepath.Join(absTenant, target))
	if err != nil {
		return "", err
	}

	// Prevent path traversal
	if absTarget == absTenant || strings.HasPrefix(absTarget, absTenant+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", fmt.Errorf("access denied: %s is outside tenant directory", target)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

// NewProvider returns the appropriate FileSystemProvider based on environment.
func NewProvider() FileSystemProvider {
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = os.TempDir()
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &CloudFSProvider{BaseDir: baseDir}
	}
	return &LocalFSProvider{BaseDir: baseDir}
}

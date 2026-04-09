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

// FileSystemProvider abstracts file system operations for MCP agents.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	BaseDir string
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	cleanBase := filepath.Clean(p.BaseDir)
	resolved := filepath.Join(cleanBase, filepath.Clean(path))
	if !strings.HasPrefix(resolved, cleanBase+string(os.PathSeparator)) && resolved != cleanBase {
		return "", fmt.Errorf("path traversal detected: %s", path)
	}
	return resolved, nil
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
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode.
type CloudFSProvider struct {
	BaseDir string
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}
	// Scope access to tenant directory
	cleanBase := filepath.Join(filepath.Clean(p.BaseDir), orgID)
	resolved := filepath.Join(cleanBase, filepath.Clean(path))

	if !strings.HasPrefix(resolved, cleanBase+string(os.PathSeparator)) && resolved != cleanBase {
		return "", fmt.Errorf("path traversal detected: %s", path)
	}

	return resolved, nil
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
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

// NewProvider returns a configured FileSystemProvider based on environment.
func NewProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &CloudFSProvider{BaseDir: "/var/ohc/tenant-volumes"}
	}
	return &LocalFSProvider{BaseDir: "/var/ohc/workspace"}
}

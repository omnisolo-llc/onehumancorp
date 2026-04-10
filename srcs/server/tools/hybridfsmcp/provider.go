package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the unified interface for file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	workspaceRoot string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(workspaceRoot string) *LocalFSProvider {
	return &LocalFSProvider{
		workspaceRoot: filepath.Clean(workspaceRoot),
	}
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	absPath := filepath.Join(p.workspaceRoot, targetPath)
	cleanPath := filepath.Clean(absPath)

	// Prevent path traversal vulnerabilities by verifying that the resolved absolute path starts with the allowed directory prefix.
	if !(strings.HasPrefix(cleanPath, p.workspaceRoot+string(filepath.Separator)) || cleanPath == p.workspaceRoot) {
		return "", errors.New("access denied: path escapes workspace boundaries")
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

	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
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
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, bounding access based on tenant ID.
type CloudFSProvider struct {
	baseStorageRoot string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseStorageRoot string) *CloudFSProvider {
	return &CloudFSProvider{
		baseStorageRoot: filepath.Clean(baseStorageRoot),
	}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, targetPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	tenantRoot := filepath.Join(p.baseStorageRoot, claims.OrganizationID)
	absPath := filepath.Join(tenantRoot, targetPath)
	cleanPath := filepath.Clean(absPath)

	if !(strings.HasPrefix(cleanPath, tenantRoot+string(filepath.Separator)) || cleanPath == tenantRoot) {
		return "", errors.New("access denied: path escapes tenant boundaries")
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

	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
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
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

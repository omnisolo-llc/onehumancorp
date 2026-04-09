package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	workspaceRoot string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to workspaceRoot.
func NewLocalFSProvider(workspaceRoot string) *LocalFSProvider {
	return &LocalFSProvider{
		workspaceRoot: filepath.Clean(workspaceRoot),
	}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	if filepath.IsAbs(target) && !strings.HasPrefix(filepath.Clean(target), p.workspaceRoot+string(filepath.Separator)) && filepath.Clean(target) != p.workspaceRoot {
		return "", fmt.Errorf("access denied: absolute path outside workspace")
	}

	cleanTarget := filepath.Clean(target)
	var fullPath string
	if filepath.IsAbs(target) {
		fullPath = cleanTarget
	} else {
		fullPath = filepath.Join(p.workspaceRoot, cleanTarget)
	}

	// Prevent directory traversal using the memory guideline check
	expectedPrefix := p.workspaceRoot + string(filepath.Separator)
	if !strings.HasPrefix(fullPath, expectedPrefix) && fullPath != p.workspaceRoot {
		return "", fmt.Errorf("access denied: path outside workspace")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Auto-create parent directories as per memory guideline
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return fmt.Errorf("failed to create parent directories: %w", err)
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode with tenant isolation.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to baseDir.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, target string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}

	if filepath.IsAbs(target) {
		return "", fmt.Errorf("access denied: absolute paths not allowed")
	}

	cleanTarget := filepath.Clean(target)
	if strings.HasPrefix(cleanTarget, "..") || strings.HasPrefix(cleanTarget, "/") {
		return "", fmt.Errorf("invalid path")
	}

	// Tenant isolation scoping
	tenantPath := filepath.Join(claims.OrganizationID, cleanTarget)
	fullPath := filepath.Join(p.baseDir, tenantPath)

	expectedPrefix := filepath.Join(p.baseDir, claims.OrganizationID) + string(filepath.Separator)
	if !strings.HasPrefix(fullPath, expectedPrefix) && fullPath != filepath.Join(p.baseDir, claims.OrganizationID) {
		return "", fmt.Errorf("access denied: path outside tenant boundary")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}

	// Auto-create parent directories
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return fmt.Errorf("failed to create parent directories: %w", err)
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]os.DirEntry, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

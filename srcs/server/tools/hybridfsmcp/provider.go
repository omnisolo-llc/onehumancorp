package hybridfsmcp

import (
	"context"
	"errors"

	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for unified file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to a base directory.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		// Fallback to the provided baseDir if absolute path resolution fails
		absBase = baseDir
	}
	return &LocalFSProvider{
		baseDir: filepath.Clean(absBase),
	}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.baseDir, path))
	if !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) && fullPath != p.baseDir {
		return "", errors.New("path traversal detected")
	}
	return fullPath, nil
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
	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider scoped by tenant.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		absBase = baseDir
	}
	return &CloudFSProvider{
		baseDir: filepath.Clean(absBase),
	}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	// The scoped path resolves relative to baseDir/organizationID
	orgBaseDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Clean(filepath.Join(orgBaseDir, path))

	if !strings.HasPrefix(fullPath, orgBaseDir+string(filepath.Separator)) && fullPath != orgBaseDir {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
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
	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

// NewFileSystemProvider is a factory to create the appropriate provider based on the environment.
func NewFileSystemProvider(isLocal bool, baseDir string) FileSystemProvider {
	if isLocal {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

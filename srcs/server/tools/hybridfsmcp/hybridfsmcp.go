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
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	BaseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider with the given base directory.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for base dir: %v", err)
	}
	return &LocalFSProvider{BaseDir: absBaseDir}, nil
}

// resolveAndCheckPath resolves the given path against BaseDir and ensures it does not escape.
func (p *LocalFSProvider) resolveAndCheckPath(pPath string) (string, error) {
	fullPath := filepath.Join(p.BaseDir, pPath)
	cleanPath := filepath.Clean(fullPath)

	baseDirWithSep := p.BaseDir + string(filepath.Separator)
	if cleanPath != p.BaseDir && !strings.HasPrefix(cleanPath, baseDirWithSep) {
		return "", fmt.Errorf("path traversal denied: %s escapes %s", pPath, p.BaseDir)
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	cleanPath, err := p.resolveAndCheckPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(cleanPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	cleanPath, err := p.resolveAndCheckPath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(cleanPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(cleanPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	cleanPath, err := p.resolveAndCheckPath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(cleanPath)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode, scoping access by tenant (OrganizationID).
type CloudFSProvider struct {
	BaseMount string
}

// NewCloudFSProvider creates a new CloudFSProvider with the given base mount directory.
func NewCloudFSProvider(baseMount string) (*CloudFSProvider, error) {
	absBaseMount, err := filepath.Abs(baseMount)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for base mount: %v", err)
	}
	return &CloudFSProvider{BaseMount: absBaseMount}, nil
}

// resolveAndCheckTenantPath resolves the path for the tenant and ensures it does not escape the tenant's directory.
func (p *CloudFSProvider) resolveAndCheckTenantPath(ctx context.Context, pPath string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID in context")
	}

	tenantDir := filepath.Join(p.BaseMount, orgID)
	fullPath := filepath.Join(tenantDir, pPath)
	cleanPath := filepath.Clean(fullPath)

	tenantDirWithSep := tenantDir + string(filepath.Separator)
	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDirWithSep) {
		return "", fmt.Errorf("path traversal denied: %s escapes tenant dir %s", pPath, tenantDir)
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	cleanPath, err := p.resolveAndCheckTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(cleanPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	cleanPath, err := p.resolveAndCheckTenantPath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(cleanPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(cleanPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	cleanPath, err := p.resolveAndCheckTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(cleanPath)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

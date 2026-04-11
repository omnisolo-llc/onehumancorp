package mcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for file system operations exposed via MCP.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider mapping directly to the local file system
// with a chrooted base directory to prevent escaping.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the given base directory.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for base dir: %w", err)
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	// Prevent absolute path escapes by ensuring target is joined properly or stripping leading slash
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", errors.New("access denied: absolute paths are not allowed")
	}
	fullPath := filepath.Join(p.baseDir, cleanTarget)

	// Ensure the full path does not escape the base directory
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes bounded directory")
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

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries where info cannot be read
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider mapping to tenant-scoped virtual directories.
// It scopes access based on the organization_id present in the auth.Claims of the context.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider with a given root directory.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for base dir: %w", err)
	}
	return &CloudFSProvider{baseDir: absBase}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", errors.New("access denied: missing organization_id in context")
	}

	tenantBaseDir := filepath.Join(p.baseDir, orgID)

	// Prevent absolute path escapes by ensuring target is joined properly or stripping leading slash
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", errors.New("access denied: absolute paths are not allowed")
	}
	fullPath := filepath.Join(tenantBaseDir, cleanTarget)

	// Ensure the full path does not escape the tenant's base directory
	if fullPath != tenantBaseDir && !strings.HasPrefix(fullPath, tenantBaseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes tenant directory")
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

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries where info cannot be read
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// NewFileSystemProvider creates an appropriate FileSystemProvider based on the environment.
// If OHC_MULTITENANT is set to true, it returns a CloudFSProvider.
// Otherwise, it returns a LocalFSProvider.
func NewFileSystemProvider() (FileSystemProvider, error) {
	// Use OHC_FS_ROOT for the root path as per the memory guidelines, fallback to temp dir.
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = os.TempDir()
	}

	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"
	if isMultiTenant {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

package hybridfsmcp

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

var (
	ErrAccessDenied = errors.New("access denied: path escapes bounds")
	ErrUnauthorized = errors.New("unauthorized: missing or invalid claims")
)

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

// LocalFSProvider implements the FileSystemProvider for standalone local mode
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the specified base directory
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to resolve absolute path for base directory: %w", err)
	}

	// Ensure the base directory exists
	if err := os.MkdirAll(absBaseDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create base directory: %w", err)
	}

	return &LocalFSProvider{
		baseDir: absBaseDir,
	}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("%w: absolute paths are not allowed", ErrAccessDenied)
	}

	fullPath := filepath.Join(p.baseDir, cleanPath)
	cleanFullPath := filepath.Clean(fullPath)

	rel, err := filepath.Rel(p.baseDir, cleanFullPath)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", ErrAccessDenied
	}

	return cleanFullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure the parent directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return fmt.Errorf("failed to create parent directory: %w", err)
	}

	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolvedPath)
}

// CloudFSProvider implements the FileSystemProvider for multi-tenant cloud mode
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to the specified base directory
// where tenant data is stored.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to resolve absolute path for base directory: %w", err)
	}

	// Ensure the root base directory exists
	if err := os.MkdirAll(absBaseDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create base directory: %w", err)
	}

	return &CloudFSProvider{
		baseDir: absBaseDir,
	}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		// Try the test key if standard key fails
		claims, _ = ctx.Value(auth.ClaimsContextKeyForTest).(*auth.Claims)
		if claims == nil {
			return "", ErrUnauthorized
		}
	}

	orgID := claims.OrganizationID
	if orgID == "" {
		return "", fmt.Errorf("%w: organization ID is missing from claims", ErrUnauthorized)
	}

	cleanPath := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("%w: absolute paths are not allowed", ErrAccessDenied)
	}

	tenantBaseDir := filepath.Join(p.baseDir, orgID)
	fullPath := filepath.Join(tenantBaseDir, cleanPath)
	cleanFullPath := filepath.Clean(fullPath)

	rel, err := filepath.Rel(tenantBaseDir, cleanFullPath)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", ErrAccessDenied
	}

	return cleanFullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure the parent directory exists
	if err := os.MkdirAll(filepath.Dir(resolvedPath), 0755); err != nil {
		return fmt.Errorf("failed to create parent directory: %w", err)
	}

	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolvedPath)
}

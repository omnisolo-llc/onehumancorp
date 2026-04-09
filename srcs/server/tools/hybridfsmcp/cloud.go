package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for the cloud environment.
// It scopes file operations to a tenant-specific virtual directory within a root directory.
type CloudFSProvider struct {
	rootDir string
}

// NewCloudFSProvider creates a new CloudFSProvider backed by the given root directory.
func NewCloudFSProvider(rootDir string) (*CloudFSProvider, error) {
	absRoot, err := filepath.Abs(rootDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for root dir: %w", err)
	}
	// Ensure the directory exists
	if err := os.MkdirAll(absRoot, 0700); err != nil {
		return nil, fmt.Errorf("failed to create root directory: %w", err)
	}
	return &CloudFSProvider{rootDir: absRoot}, nil
}

// resolvePath resolves the given path against the tenant-specific directory and ensures it does not escape.
func (p *CloudFSProvider) resolvePath(claims *auth.Claims, targetPath string) (string, error) {
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	tenantDir := filepath.Join(p.rootDir, claims.OrganizationID)

	// Create tenant directory if it doesn't exist
	if err := os.MkdirAll(tenantDir, 0700); err != nil {
		return "", fmt.Errorf("failed to create tenant directory: %w", err)
	}

	cleanInput := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanInput) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", targetPath)
	}

	absTarget := filepath.Join(tenantDir, cleanInput)
	cleanTarget := filepath.Clean(absTarget)

	// Check if the resolved path is within the tenant directory
	rel, err := filepath.Rel(tenantDir, cleanTarget)
	if err != nil {
		return "", fmt.Errorf("failed to determine relative path: %w", err)
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes tenant directory: %s", targetPath)
	}

	return cleanTarget, nil
}

// ReadFile reads the file at the given path for the specific tenant.
func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

// WriteFile writes the given content to the file at the given path for the specific tenant.
func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	resolvedPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return fmt.Errorf("failed to create parent directories: %w", err)
	}

	return os.WriteFile(resolvedPath, content, 0600)
}

// ListDir lists the files in the directory at the given path for the specific tenant.
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

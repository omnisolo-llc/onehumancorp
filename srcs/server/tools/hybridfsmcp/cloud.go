package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for cloud/multitenant environments
type CloudFSProvider struct {
	baseDir string // e.g., a persistent volume mount point
}

// NewCloudFSProvider creates a new cloud filesystem provider
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path of base directory: %w", err)
	}
	return &CloudFSProvider{baseDir: absBaseDir}, nil
}

// resolvePath returns the tenant-scoped path.
// It requires an authenticated context with auth.Claims to determine the OrganizationID.
func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthenticated or missing organization ID in context")
	}

	tenantDir := filepath.Join(p.baseDir, "tenants", claims.OrganizationID)

	cleanPath := filepath.Clean(path)
	absPath := filepath.Join(tenantDir, cleanPath)

	rel, err := filepath.Rel(tenantDir, absPath)
	// Ensure the resolved path doesn't escape the tenant's directory
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path escapes tenant directory: %s", path)
	}

	return absPath, nil
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

	// Create parent directories if they don't exist
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return fmt.Errorf("failed to create parent directories: %w", err)
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read directory: %w", err)
	}

	var result []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries where we can't get info
		}
		result = append(result, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}

	return result, nil
}

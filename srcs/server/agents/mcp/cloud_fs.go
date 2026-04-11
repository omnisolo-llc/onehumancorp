package mcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider. It constructs tenant-scoped paths
// dynamically based on the context's auth.Claims to provide strict multi-tenant isolation.
type CloudFSProvider struct {
	baseDir string // e.g., "/mnt/pv"
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for baseDir: %w", err)
	}

	return &CloudFSProvider{
		baseDir: absBase,
	}, nil
}

// resolvePath extracts the OrganizationID from the context, scopes the path to that tenant,
// and ensures it does not escape the tenant's bounded directory.
func (p *CloudFSProvider) resolvePath(ctx context.Context, targetPath string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", errors.New("unauthorized: missing organization ID in context")
	}

	// Create the tenant's bounded directory within the global base dir
	tenantDir := filepath.Join(p.baseDir, fmt.Sprintf("tenant-%s", orgID))

	// Join the targetPath relative to the tenant's root
	fullPath := filepath.Join(tenantDir, targetPath)
	cleanPath := filepath.Clean(fullPath)

	// Path traversal check scoped to the tenant's directory
	tenantDirWithSep := tenantDir + string(filepath.Separator)
	if !strings.HasPrefix(cleanPath, tenantDirWithSep) && cleanPath != tenantDir {
		return "", errors.New("path traversal violation: target path escapes tenant directory")
	}

	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	return os.ReadFile(resolvedPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolvedPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolvedPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(ctx, path)
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

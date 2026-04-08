package mcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider with tenant-scoped access.
// It relies on auth.Claims from the context to determine the organization_id
// and chroots access into a tenant-specific virtual directory within the mount.
type CloudFSProvider struct {
	baseMount string
}

func NewCloudFSProvider(baseMount string) (*CloudFSProvider, error) {
	absMount, err := filepath.Abs(baseMount)
	if err != nil {
		return nil, fmt.Errorf("invalid base mount directory: %w", err)
	}
	return &CloudFSProvider{baseMount: absMount}, nil
}

// resolvePath securely resolves a path, ensuring it stays within the tenant's bounded directory
func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims in context")
	}

	tenantDir := filepath.Join(p.baseMount, claims.OrganizationID)

	// Ensure tenant directory exists on resolution
	if err := os.MkdirAll(tenantDir, 0755); err != nil {
		return "", fmt.Errorf("failed to ensure tenant directory exists: %w", err)
	}

	absPath, err := filepath.Abs(filepath.Join(tenantDir, target))
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}

	// Ensure prefix matching ends with separator to prevent prefix spoofing
	tenantDirWithSep := tenantDir
	if !strings.HasSuffix(tenantDirWithSep, string(filepath.Separator)) {
		tenantDirWithSep += string(filepath.Separator)
	}
	absPathWithSep := absPath
	if !strings.HasSuffix(absPathWithSep, string(filepath.Separator)) {
		absPathWithSep += string(filepath.Separator)
	}

	// It's safe if it perfectly matches the tenant dir, or has it as prefix
	if absPath != tenantDir && !strings.HasPrefix(absPathWithSep, tenantDirWithSep) {
		return "", fmt.Errorf("access denied: path escapes tenant bounds")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure parent directory exists
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, fmt.Errorf("failed to read directory: %w", err)
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

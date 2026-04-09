package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for cloud mode,
// scoping file operations by tenant (OrganizationID).
type CloudFSProvider struct {
	basePVMount string // e.g., /mnt/tenant_data
}

// NewCloudFSProvider creates a new CloudFSProvider with a base PV mount.
func NewCloudFSProvider(basePVMount string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(basePVMount)
	if err != nil {
		return nil, fmt.Errorf("invalid base PV mount: %w", err)
	}
	return &CloudFSProvider{basePVMount: absBase}, nil
}

// resolvePath scopes the path to the tenant's subdirectory.
func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		// Fallback for tests or explicit injection
		if c, ok := ctx.Value("auth_claims_test_fallback").(*auth.Claims); ok {
			claims = c
		} else {
			return "", fmt.Errorf("unauthorized: missing claims in context")
		}
	}

	orgID := claims.OrganizationID
	if orgID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID in claims")
	}

	tenantDir := filepath.Join(p.basePVMount, orgID)

	// Create clean target path relative to tenant directory
	cleanTarget := filepath.Clean(target)
	fullPath := filepath.Join(tenantDir, cleanTarget)

	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil {
		return "", fmt.Errorf("path access denied: invalid path")
	}

	if strings.HasPrefix(rel, "..") || strings.HasPrefix(rel, string(filepath.Separator)) {
		return "", fmt.Errorf("path access denied: escapes tenant directory")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure tenant structure exists
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create tenant directory: %w", err)
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}
	return infos, nil
}

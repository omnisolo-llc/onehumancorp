package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider for a cloud environment (e.g., K8s persistent volume).
// It scopes access based on the tenant's organization ID.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to a base directory.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{
		baseDir: absBase,
	}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	cleanTarget := filepath.Clean(target)
	if strings.HasPrefix(cleanTarget, "/") {
		cleanTarget = strings.TrimPrefix(cleanTarget, "/")
	}

	// Tenant scoped path: /baseDir/tenantID/targetPath
	tenantBase := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantBase, cleanTarget)

	// Ensure the path doesn't escape the tenant's base directory
	if !strings.HasPrefix(fullPath, tenantBase+string(filepath.Separator)) && fullPath != tenantBase {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}

// ReadFile reads a file from the cloud file system.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

// WriteFile writes data to a file in the cloud file system.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

// ListDir lists contents of a directory in the cloud file system.
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
			continue // Skip entries we can't get info for
		}
		infos = append(infos, info)
	}
	return infos, nil
}

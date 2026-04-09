package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider implements FileSystemProvider by mapping requests to a
// tenant-scoped directory. It uses auth.Claims from the context to extract
// the OrganizationID and scopes all operations to that specific directory.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{
		baseDir: absBase,
	}, nil
}

// resolvePath resolves the given path against the base directory and ensures
// that it does not escape the tenant's isolated directory.
func (p *CloudFSProvider) resolvePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	cleanTarget := filepath.Clean(targetPath)

	// Prevent absolute paths from escaping
	if filepath.IsAbs(cleanTarget) {
		cleanTarget = strings.TrimPrefix(cleanTarget, "/")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanTarget)

	// Clean the resulting path again to resolve any . or ..
	fullPath = filepath.Clean(fullPath)

	if !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) && fullPath != tenantDir {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}

// ReadFile reads the contents of the file at the specified path.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	return os.ReadFile(fullPath)
}

// WriteFile writes the given data to the file at the specified path.
// It creates the parent directories if they do not exist.
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

// ListDir lists the entries of the directory at the specified path.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		names = append(names, name)
	}

	return names, nil
}

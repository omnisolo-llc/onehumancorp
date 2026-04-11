package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	if baseDir == "" {
		baseDir = os.Getenv("OHC_FS_ROOT")
		if baseDir == "" {
			baseDir = os.TempDir()
		}
	}
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, target))
	// Validating directory boundaries using target == base || strings.HasPrefix(target, base+string(filepath.Separator))
	if cleanPath != p.baseDir && !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}
	return cleanPath, nil
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
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, bounding access to tenant-scoped virtual directories.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	if baseDir == "" {
		baseDir = os.Getenv("OHC_FS_ROOT")
		if baseDir == "" {
			baseDir = "/mnt/tenant-data" // standard k8s persistent volume mount point
		}
	}
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, target))

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}
	return cleanPath, nil
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
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return []string{}, nil
		}
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

// NewProviderFactory returns the appropriate provider based on OHC_MULTITENANT.
func NewProviderFactory(baseDir string) FileSystemProvider {
	if strings.ToLower(os.Getenv("OHC_MULTITENANT")) == "true" {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

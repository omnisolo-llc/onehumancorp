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

var (
	ErrAccessDenied = errors.New("access denied: path escapes bounded directory")
	ErrNotImplemented = errors.New("not implemented")
)

// FileSystemProvider defines the unified interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte, mode fs.FileMode) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode
type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanBase := filepath.Clean(p.BaseDir)
	cleanTarget := filepath.Clean(filepath.Join(cleanBase, target))

	if !(cleanTarget == cleanBase || strings.HasPrefix(cleanTarget, cleanBase+string(filepath.Separator))) {
		return "", ErrAccessDenied
	}
	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte, mode fs.FileMode) error {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, mode)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode
type CloudFSProvider struct {
	VolumeRoot string
}

func NewCloudFSProvider(volumeRoot string) *CloudFSProvider {
	return &CloudFSProvider{VolumeRoot: volumeRoot}
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx); ok := claims != nil
	if !ok || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant context")
	}

	tenantBase := filepath.Join(p.VolumeRoot, claims.OrganizationID)
	cleanBase := filepath.Clean(tenantBase)
	cleanTarget := filepath.Clean(filepath.Join(cleanBase, target))

	if !(cleanTarget == cleanBase || strings.HasPrefix(cleanTarget, cleanBase+string(filepath.Separator))) {
		return "", ErrAccessDenied
	}
	return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte, mode fs.FileMode) error {
	safePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, mode)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	safePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

// Factory to get correct provider
func NewProvider(ctx context.Context) (FileSystemProvider, error) {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultiTenant {
		root := os.Getenv("OHC_CLOUD_VOLUME_ROOT")
		if root == "" {
			root = "/mnt/data/tenants" // default cloud pv
		}
		return NewCloudFSProvider(root), nil
	}

	// Standalone local mode
	root := os.Getenv("OHC_LOCAL_WORKSPACE")
	if root == "" {
		root = filepath.Join(os.Getenv("HOME"), ".ohc-workspace")
	}
	return NewLocalFSProvider(root), nil
}

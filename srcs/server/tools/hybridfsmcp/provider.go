package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for Standalone (local) mode.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{
		workspaceDir: workspaceDir,
	}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	basePath := filepath.Clean(p.workspaceDir)
	cleanPath := filepath.Clean(filepath.Join(basePath, path))

	// Ensure the base path ends with a separator to prevent prefix sharing attacks
	basePathWithSep := basePath
	if !strings.HasSuffix(basePathWithSep, string(filepath.Separator)) {
		basePathWithSep += string(filepath.Separator)
	}

	if cleanPath != basePath && !strings.HasPrefix(cleanPath, basePathWithSep) {
		return "", errors.New("path escapes workspace directory")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var res []FileEntry
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		res = append(res, FileEntry{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: baseDir,
	}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	tenantDir := filepath.Clean(filepath.Join(p.baseDir, claims.OrganizationID))
	cleanPath := filepath.Clean(filepath.Join(tenantDir, path))

	// Ensure the tenant directory ends with a separator to prevent prefix sharing attacks
	tenantDirWithSep := tenantDir
	if !strings.HasSuffix(tenantDirWithSep, string(filepath.Separator)) {
		tenantDirWithSep += string(filepath.Separator)
	}

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDirWithSep) {
		return "", errors.New("path escapes tenant directory")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileEntry, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var res []FileEntry
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		res = append(res, FileEntry{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

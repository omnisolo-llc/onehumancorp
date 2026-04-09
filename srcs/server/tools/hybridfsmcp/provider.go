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

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider bounded to a local workspace
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, targetPath))
	if !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) && cleanPath != p.baseDir {
		return "", fmt.Errorf("path access denied: out of bounds")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(path)
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
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider bounded to tenant-scoped virtual directories
type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{baseStorageDir: filepath.Clean(baseStorageDir)}
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, targetPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}

	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) && cleanPath != tenantDir {
		return "", fmt.Errorf("path access denied: cross-tenant access forbidden")
	}

	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolved, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(claims, path)
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
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

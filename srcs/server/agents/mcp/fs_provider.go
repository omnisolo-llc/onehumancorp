package mcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileInfo represents metadata about a file or directory.
type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

// FileSystemProvider abstracts file reading, writing, and directory listing.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

// LocalFSProvider implements FileSystemProvider mapping to local file system with safety bounds.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *LocalFSProvider) validatePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, targetPath))
	if cleanPath == p.baseDir || strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", errors.New("path escapes base directory")
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.validatePath(path)
	if err != nil {
		return err
	}

	// Create directories if needed
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip if we can't get info
		}
		infos = append(infos, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return infos, nil
}

// CloudFSProvider implements FileSystemProvider, tenant-scoped using auth.Claims.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *CloudFSProvider) getTenantPath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	if claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization_id")
	}

	tenantDir := filepath.Clean(filepath.Join(p.baseDir, claims.OrganizationID))
	cleanPath := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if cleanPath == tenantDir || strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", errors.New("path escapes tenant directory")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return err
	}

	// Create directories if needed
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	safePath, err := p.getTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return []FileInfo{}, nil
		}
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return infos, nil
}

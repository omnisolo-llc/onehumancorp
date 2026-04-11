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

// FileInfo contains metadata about a file or directory.
type FileInfo struct {
	Name    string
	Size    int64
	IsDir   bool
	ModTime string
}

// FileSystemProvider abstracts file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	if baseDir == "" {
		baseDir = "."
	}
	absPath, _ := filepath.Abs(baseDir)
	return &LocalFSProvider{baseDir: absPath}
}

// resolvePath safely resolves a path relative to the base directory.
func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(reqPath)
	fullPath := filepath.Join(p.baseDir, cleanPath)
	if !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) && fullPath != p.baseDir {
		return "", errors.New("path escapes base directory")
	}
	return fullPath, nil
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var res []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		res = append(res, FileInfo{
			Name:    e.Name(),
			Size:    info.Size(),
			IsDir:   e.IsDir(),
			ModTime: info.ModTime().Format("2006-01-02T15:04:05Z07:00"),
		})
	}
	return res, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant scoping.
// In reality this might be backed by a PV or S3, but we use local file system to simulate PV.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	if baseDir == "" {
		baseDir = "/tmp/ohc_cloud_fs"
	}
	absPath, _ := filepath.Abs(baseDir)
	return &CloudFSProvider{baseDir: absPath}
}

// resolvePath safely resolves a path for a given tenant.
func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(reqPath)
	fullPath := filepath.Join(tenantDir, cleanPath)
	if !strings.HasPrefix(fullPath, tenantDir+string(filepath.Separator)) && fullPath != tenantDir {
		return "", errors.New("path escapes tenant directory")
	}
	return fullPath, nil
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		// If dir doesn't exist yet for tenant, return empty list instead of error
		if errors.Is(err, fs.ErrNotExist) {
			return []FileInfo{}, nil
		}
		return nil, err
	}
	var res []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		res = append(res, FileInfo{
			Name:    e.Name(),
			Size:    info.Size(),
			IsDir:   e.IsDir(),
			ModTime: info.ModTime().Format("2006-01-02T15:04:05Z07:00"),
		})
	}
	return res, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// NewFileSystemProvider creates the appropriate provider based on the environment.
func NewFileSystemProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

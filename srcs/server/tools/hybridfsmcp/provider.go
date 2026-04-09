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

// FileInfo provides metadata about a file or directory.
type FileInfo struct {
	Name    string
	Size    int64
	IsDir   bool
	ModTime string
}

// FileSystemProvider abstracts file operations for the HybridFS MCP.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, restricted to a base directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, reqPath))
	prefix := p.baseDir
	if !strings.HasSuffix(prefix, string(filepath.Separator)) {
		prefix += string(filepath.Separator)
	}
	if !strings.HasPrefix(cleanPath, prefix) && cleanPath != p.baseDir {
		return "", fmt.Errorf("access denied: path outside base directory")
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
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, FileInfo{
			Name:    entry.Name(),
			Size:    info.Size(),
			IsDir:   entry.IsDir(),
			ModTime: info.ModTime().String(),
		})
	}
	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error) {
	var matches []string
	err := filepath.WalkDir(p.baseDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			relPath, err := filepath.Rel(p.baseDir, path)
			if err == nil && (strings.Contains(filepath.Base(path), query) || strings.Contains(relPath, query)) {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, tenant-scoped.
type CloudFSProvider struct {
	virtualBaseDir string
}

func NewCloudFSProvider(virtualBaseDir string) *CloudFSProvider {
	return &CloudFSProvider{virtualBaseDir: virtualBaseDir}
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, reqPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("access denied: missing tenant claims")
	}
	tenantDir := filepath.Join(p.virtualBaseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, reqPath))
	prefix := tenantDir
	if !strings.HasSuffix(prefix, string(filepath.Separator)) {
		prefix += string(filepath.Separator)
	}
	if !strings.HasPrefix(cleanPath, prefix) && cleanPath != tenantDir {
		return "", fmt.Errorf("access denied: path outside tenant directory")
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
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, FileInfo{
			Name:    entry.Name(),
			Size:    info.Size(),
			IsDir:   entry.IsDir(),
			ModTime: info.ModTime().String(),
		})
	}
	return infos, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return nil, fmt.Errorf("access denied: missing tenant claims")
	}
	tenantDir := filepath.Join(p.virtualBaseDir, claims.OrganizationID)

	var matches []string
	err := filepath.WalkDir(tenantDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			relPath, err := filepath.Rel(tenantDir, path)
			if err == nil && (strings.Contains(filepath.Base(path), query) || strings.Contains(relPath, query)) {
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

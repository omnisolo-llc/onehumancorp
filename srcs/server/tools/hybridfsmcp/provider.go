package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

type LocalFSProvider struct {
	WorkspaceDir string
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanPath := filepath.Clean(target)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	base := filepath.Clean(p.WorkspaceDir)
	fullPath := filepath.Join(base, cleanPath)

	rel, err := filepath.Rel(base, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path traversal denied")
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

	// Create directory structure if it doesn't exist
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}

	// Open file with 0600 permissions
	f, err := os.OpenFile(fullPath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		return err
	}
	defer f.Close()

	_, err = f.Write(data)
	return err
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

	result := make([]FileInfo, 0)
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		result = append(result, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return result, nil
}

type CloudFSProvider struct {
	BaseDir string
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant context")
	}

	tenantClean := filepath.Clean(claims.OrganizationID)
	tenantDir := filepath.Join(p.BaseDir, tenantClean)

	cleanPath := filepath.Clean(target)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	fullPath := filepath.Join(tenantDir, cleanPath)
	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path traversal denied")
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

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}

	f, err := os.OpenFile(fullPath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		return err
	}
	defer f.Close()

	_, err = f.Write(data)
	return err
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	result := make([]FileInfo, 0)
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		result = append(result, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return result, nil
}

func NewFileSystemProvider(isStandalone bool, baseDir string) FileSystemProvider {
	if isStandalone {
		return &LocalFSProvider{WorkspaceDir: baseDir}
	}
	return &CloudFSProvider{BaseDir: baseDir}
}

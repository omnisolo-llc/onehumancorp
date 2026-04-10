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
	Name  string `json:"name"`
	IsDir bool   `json:"is_dir"`
	Size  int64  `json:"size"`
}

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) (string, error)
	WriteFile(ctx context.Context, path string, content string) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider() *LocalFSProvider {
	baseDir := os.Getenv("OHC_WORKSPACE_DIR")
	if baseDir == "" {
		baseDir = "/tmp/ohc_workspace"
	}
	// Ensure base dir exists
	os.MkdirAll(baseDir, 0700)
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	if filepath.IsAbs(reqPath) {
		return "", fmt.Errorf("access denied: absolute path not allowed")
	}
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, reqPath))
	// Prevent directory traversal
	if !strings.HasPrefix(cleanPath, filepath.Clean(p.baseDir)+string(filepath.Separator)) && cleanPath != p.baseDir {
		return "", fmt.Errorf("access denied: invalid path")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) (string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return "", err
	}
	data, err := os.ReadFile(fullPath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content string) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// ensure dir exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(fullPath, []byte(content), 0644)
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
		info, _ := e.Info()
		res = append(res, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider() *CloudFSProvider {
	baseDir := os.Getenv("OHC_TENANT_PV_DIR")
	if baseDir == "" {
		baseDir = "/tmp/ohc_pv"
	}
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}
	if filepath.IsAbs(reqPath) {
		return "", fmt.Errorf("access denied: absolute path not allowed")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	// Ensure tenant dir exists
	os.MkdirAll(tenantDir, 0700)

	cleanPath := filepath.Clean(filepath.Join(tenantDir, reqPath))
	if !strings.HasPrefix(cleanPath, filepath.Clean(tenantDir)+string(filepath.Separator)) && cleanPath != tenantDir {
		return "", fmt.Errorf("access denied: invalid path")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) (string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return "", err
	}
	data, err := os.ReadFile(fullPath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content string) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	// ensure dir exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(fullPath, []byte(content), 0644)
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
	var res []FileInfo
	for _, e := range entries {
		info, _ := e.Info()
		res = append(res, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

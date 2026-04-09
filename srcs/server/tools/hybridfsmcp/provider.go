package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileInfo struct {
	Name    string    `json:"name"`
	Size    int64     `json:"size"`
	IsDir   bool      `json:"is_dir"`
	ModTime time.Time `json:"mod_time"`
}

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	if workspaceDir == "" {
		workspaceDir = "."
	}
	absPath, err := filepath.Abs(workspaceDir)
	if err == nil {
		workspaceDir = absPath
	}
	return &LocalFSProvider{
		workspaceDir: workspaceDir,
	}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", reqPath)
	}

	fullPath := filepath.Join(p.workspaceDir, cleanPath)

	// Ensure the resolved path is within the workspace dir
	baseDirWithSep := p.workspaceDir
	if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
		baseDirWithSep += string(filepath.Separator)
	}

	if fullPath != p.workspaceDir && !strings.HasPrefix(fullPath, baseDirWithSep) {
		return "", fmt.Errorf("path traversal attempt detected: %s", reqPath)
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure parent directory exists
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // skip entries we can't stat
		}
		infos = append(infos, FileInfo{
			Name:    entry.Name(),
			Size:    info.Size(),
			IsDir:   entry.IsDir(),
			ModTime: info.ModTime(),
		})
	}
	return infos, nil
}

type CloudFSProvider struct {
	pvDir string
}

func NewCloudFSProvider(pvDir string) *CloudFSProvider {
	if pvDir == "" {
		pvDir = "/mnt/tenant-data"
	}
	return &CloudFSProvider{
		pvDir: pvDir,
	}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", fmt.Errorf("unauthorized: missing claims in cloud mode")
	}

	if claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}

	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", reqPath)
	}

	// Tenant scoped virtual directory
	tenantDir := filepath.Join(p.pvDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanPath)

	tenantDirWithSep := tenantDir
	if !strings.HasSuffix(tenantDirWithSep, string(filepath.Separator)) {
		tenantDirWithSep += string(filepath.Separator)
	}

	if fullPath != tenantDir && !strings.HasPrefix(fullPath, tenantDirWithSep) {
		return "", fmt.Errorf("path traversal attempt detected: %s", reqPath)
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
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
			ModTime: info.ModTime(),
		})
	}
	return infos, nil
}

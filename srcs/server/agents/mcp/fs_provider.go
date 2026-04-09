package mcp

import (
	"context"
	"errors"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) (string, error)
	WriteFile(ctx context.Context, path string, content string) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
	rootDir string
}

func NewLocalFSProvider() *LocalFSProvider {
	dir := os.Getenv("OHC_WORKSPACE_DIR")
	if dir == "" {
		dir = "/tmp/workspace"
	}
	return &LocalFSProvider{rootDir: filepath.Clean(dir)}
}

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
	cleanTarget := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanTarget) {
		return "", errors.New("absolute paths are not allowed")
	}
	fullPath := filepath.Join(p.rootDir, cleanTarget)
	baseWithSep := p.rootDir
	if !strings.HasSuffix(baseWithSep, string(filepath.Separator)) {
		baseWithSep += string(filepath.Separator)
	}
	if !strings.HasPrefix(fullPath, baseWithSep) && fullPath != p.rootDir {
		return "", errors.New("path traversal detected")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) (string, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return "", err
	}
	data, err := os.ReadFile(secure)
	return string(data), err
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content string) error {
	secure, err := p.securePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, []byte(content), 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	var files []string
	entries, err := os.ReadDir(secure)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return []string{}, nil
		}
		return nil, err
	}
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

type CloudFSProvider struct {
	baseTenantDir string
}

func NewCloudFSProvider() *CloudFSProvider {
	dir := os.Getenv("OHC_TENANT_PV_DIR")
	if dir == "" {
		dir = "/mnt/tenant_pv"
	}
	return &CloudFSProvider{baseTenantDir: filepath.Clean(dir)}
}

func (p *CloudFSProvider) securePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization claims")
	}
	tenantRoot := filepath.Join(p.baseTenantDir, claims.OrganizationID)
	cleanTarget := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanTarget) {
		return "", errors.New("absolute paths are not allowed")
	}
	fullPath := filepath.Join(tenantRoot, cleanTarget)
	baseWithSep := tenantRoot
	if !strings.HasSuffix(baseWithSep, string(filepath.Separator)) {
		baseWithSep += string(filepath.Separator)
	}
	if !strings.HasPrefix(fullPath, baseWithSep) && fullPath != tenantRoot {
		return "", errors.New("path traversal detected")
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) (string, error) {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return "", err
	}
	data, err := os.ReadFile(secure)
	return string(data), err
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content string) error {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, []byte(content), 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	var files []string
	entries, err := os.ReadDir(secure)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return []string{}, nil
		}
		return nil, err
	}
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

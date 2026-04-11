package mcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
	fullPath := filepath.Join(p.baseDir, targetPath)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != p.baseDir && !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes base directory")
	}

	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.securePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{baseStorageDir: baseStorageDir}
}

func (p *CloudFSProvider) secureTenantPath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims in context")
	}

	tenantDir := filepath.Join(p.baseStorageDir, "tenant_"+claims.OrganizationID)

	fullPath := filepath.Join(tenantDir, targetPath)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes tenant directory")
	}

	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.secureTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.secureTenantPath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.secureTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func NewHybridFSProvider(isStandalone bool, baseDir string) FileSystemProvider {
	if isStandalone {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

package hybridfsmcp

import (
	"context"
	"fmt"
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
	BaseDir string
}

func (p *LocalFSProvider) sanitizePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.BaseDir, path))
	if filepath.IsAbs(path) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}
	baseDir := p.BaseDir
	if !strings.HasSuffix(baseDir, string(filepath.Separator)) {
		baseDir += string(filepath.Separator)
	}
	if !strings.HasPrefix(cleanPath, baseDir) && cleanPath != p.BaseDir {
		return "", fmt.Errorf("path escapes base directory")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.sanitizePath(path)
	if err != nil {
		return err
	}

	err = os.MkdirAll(filepath.Dir(safePath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.sanitizePath(path)
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
	TenantBaseDir string
}

func (p *CloudFSProvider) getTenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized or missing tenant")
	}
	return filepath.Join(p.TenantBaseDir, claims.OrganizationID), nil
}

func (p *CloudFSProvider) sanitizePath(ctx context.Context, path string) (string, error) {
	tenantDir, err := p.getTenantDir(ctx)
	if err != nil {
		return "", err
	}

	if filepath.IsAbs(path) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	cleanPath := filepath.Clean(filepath.Join(tenantDir, path))
	baseDir := tenantDir
	if !strings.HasSuffix(baseDir, string(filepath.Separator)) {
		baseDir += string(filepath.Separator)
	}
	if !strings.HasPrefix(cleanPath, baseDir) && cleanPath != tenantDir {
		return "", fmt.Errorf("path escapes tenant directory")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.sanitizePath(ctx, path)
	if err != nil {
		return err
	}

	err = os.MkdirAll(filepath.Dir(safePath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.sanitizePath(ctx, path)
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

func NewFileSystemProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		baseDir := os.Getenv("OHC_TENANT_PV_DIR")
		if baseDir == "" {
			baseDir = "/var/ohc/tenants"
		}
		return &CloudFSProvider{TenantBaseDir: baseDir}
	} else {
		baseDir := os.Getenv("OHC_WORKSPACE_DIR")
		if baseDir == "" {
			baseDir = "/tmp/ohc_workspace"
		}
		return &LocalFSProvider{BaseDir: baseDir}
	}
}

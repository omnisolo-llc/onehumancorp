package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for our unified Hybrid FS operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace directory.
type LocalFSProvider struct {
	WorkspaceDir string
}

func NewLocalFSProvider(workspace string) (*LocalFSProvider, error) {
	abs, err := filepath.Abs(workspace)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{WorkspaceDir: abs}, nil
}

func (p *LocalFSProvider) sanitizePath(reqPath string) (string, error) {
	clean := filepath.Clean(reqPath)
	fullPath := filepath.Join(p.WorkspaceDir, clean)

	// Ensure the resulting path does not escape the workspace
	// Using filepath.Separator to prevent matching prefix strings like "workspace10" when base is "workspace1"
	if fullPath != p.WorkspaceDir && !strings.HasPrefix(fullPath, p.WorkspaceDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes workspace")
	}
	return fullPath, nil
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
	// Ensure parent directories exist
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
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
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud-native mode.
// In reality, this might wrap an S3 interface or K8s PV. For this MCP, we simulate a local cloud-mounted directory
// that requires Tenant isolation via auth.Claims.
type CloudFSProvider struct {
	BaseCloudMount string
}

func NewCloudFSProvider(mount string) (*CloudFSProvider, error) {
	abs, err := filepath.Abs(mount)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{BaseCloudMount: abs}, nil
}

func (p *CloudFSProvider) tenantDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: tenant context missing")
	}
	return filepath.Join(p.BaseCloudMount, claims.OrganizationID), nil
}

func (p *CloudFSProvider) sanitizePath(ctx context.Context, reqPath string) (string, error) {
	tdir, err := p.tenantDir(ctx)
	if err != nil {
		return "", err
	}
	clean := filepath.Clean(reqPath)
	fullPath := filepath.Join(tdir, clean)

	if fullPath != tdir && !strings.HasPrefix(fullPath, tdir+string(filepath.Separator)) {
		return "", errors.New("access denied: cross-tenant access attempt")
	}
	return fullPath, nil
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
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
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
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

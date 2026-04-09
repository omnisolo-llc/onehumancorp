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

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

type LocalFSProvider struct {
	workspaceRoot string
}

func NewLocalFSProvider(workspaceRoot string) *LocalFSProvider {
	return &LocalFSProvider{workspaceRoot: filepath.Clean(workspaceRoot)}
}

func (p *LocalFSProvider) validatePath(targetPath string) (string, error) {
	absPath := filepath.Clean(filepath.Join(p.workspaceRoot, targetPath))
	if !strings.HasPrefix(absPath, p.workspaceRoot+string(filepath.Separator)) && absPath != p.workspaceRoot {
		return "", fmt.Errorf("path escapes workspace")
	}
	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	validPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(validPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	validPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(validPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(validPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	validPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(validPath)
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) validatePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized or missing organization ID")
	}


	tenantDir := filepath.Clean(filepath.Join(p.baseDir, claims.OrganizationID))
	absPath := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if !strings.HasPrefix(absPath, tenantDir+string(filepath.Separator)) && absPath != tenantDir {
		return "", fmt.Errorf("path escapes tenant directory")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	validPath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(validPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	validPath, err := p.validatePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(validPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(validPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	validPath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(validPath)
}

func NewFileSystemProvider(mode string, baseDir string) FileSystemProvider {
	if mode == "OHC_STANDALONE" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

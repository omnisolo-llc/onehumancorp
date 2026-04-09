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

type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.DirEntry, error)
}

type LocalFSProvider struct {
	rootDir string
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.rootDir, path))
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(absPath+string(filepath.Separator), p.rootDir+string(filepath.Separator)) && absPath != p.rootDir {
		return "", errors.New("path traversal detected")
	}
	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.DirEntry, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

type CloudFSProvider struct {
	baseDir string
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	absPath, err := filepath.Abs(filepath.Join(tenantDir, path))
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(absPath+string(filepath.Separator), tenantDir+string(filepath.Separator)) && absPath != tenantDir {
		return "", errors.New("path traversal detected")
	}
	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	absPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	absPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.DirEntry, error) {
	absPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

func NewFileSystemProvider(isCloud bool, rootDir string) FileSystemProvider {
	absRoot, _ := filepath.Abs(rootDir)
	if isCloud {
		return &CloudFSProvider{baseDir: absRoot}
	}
	return &LocalFSProvider{rootDir: absRoot}
}

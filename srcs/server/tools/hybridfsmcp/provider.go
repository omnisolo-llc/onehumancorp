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

var (
	ErrAccessDenied = errors.New("access denied")
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error)
}

type LocalFSProvider struct {
	fsRoot string
}

func NewLocalFSProvider(root string) *LocalFSProvider {
	return &LocalFSProvider{fsRoot: root}
}

func (p *LocalFSProvider) validatePath(targetPath string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.fsRoot, targetPath))
	if err != nil {
		return "", err
	}
	absRoot, err := filepath.Abs(p.fsRoot)
	if err != nil {
		return "", err
	}
	if absTarget == absRoot || strings.HasPrefix(absTarget, absRoot+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", ErrAccessDenied
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	validPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(validPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	validPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(validPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(validPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	validPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(validPath)
	if err != nil {
		return nil, err
	}
	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error) {
	validPath, err := p.validatePath(".")
	if err != nil {
		return nil, err
	}
	var results []string
	err = filepath.WalkDir(validPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() && strings.Contains(filepath.Base(path), query) {
			rel, _ := filepath.Rel(validPath, path)
			results = append(results, rel)
		}
		return nil
	})
	return results, err
}

type CloudFSProvider struct {
	fsRoot string
}

func NewCloudFSProvider(root string) *CloudFSProvider {
	return &CloudFSProvider{fsRoot: root}
}

func (p *CloudFSProvider) validatePath(claims *auth.Claims, targetPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", ErrAccessDenied
	}
	tenantRoot := filepath.Join(p.fsRoot, claims.OrganizationID)
	absTarget, err := filepath.Abs(filepath.Join(tenantRoot, targetPath))
	if err != nil {
		return "", err
	}
	absRoot, err := filepath.Abs(tenantRoot)
	if err != nil {
		return "", err
	}
	if absTarget == absRoot || strings.HasPrefix(absTarget, absRoot+string(filepath.Separator)) {
		return absTarget, nil
	}
	return "", ErrAccessDenied
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	validPath, err := p.validatePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(validPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	validPath, err := p.validatePath(claims, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(validPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(validPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	validPath, err := p.validatePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(validPath)
	if err != nil {
		return nil, err
	}
	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return nil, ErrAccessDenied
	}
	validPath, err := p.validatePath(claims, ".")
	if err != nil {
		return nil, err
	}
	var results []string
	err = filepath.WalkDir(validPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() && strings.Contains(filepath.Base(path), query) {
			rel, _ := filepath.Rel(validPath, path)
			results = append(results, rel)
		}
		return nil
	})
	return results, err
}

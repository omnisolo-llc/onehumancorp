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
	WorkspaceDir string
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	if filepath.IsAbs(filepath.Clean(target)) {
		return "", errors.New("absolute paths are not allowed")
	}
	fullPath := filepath.Join(p.WorkspaceDir, target)
	rel, err := filepath.Rel(p.WorkspaceDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", errors.New("path traversal detected")
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return []string{}, nil
		}
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	BaseDir string
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant organization ID")
	}

	if filepath.IsAbs(filepath.Clean(target)) {
		return "", errors.New("absolute paths are not allowed")
	}

	tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, target)

	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", errors.New("path traversal detected")
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
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return []string{}, nil
		}
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

func NewFileSystemProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &CloudFSProvider{BaseDir: baseDir}
	}
	return &LocalFSProvider{WorkspaceDir: baseDir}
}

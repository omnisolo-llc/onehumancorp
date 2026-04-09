package mcp

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
	ErrPathTraversal = errors.New("path traversal detected")
	ErrUnauthorized  = errors.New("unauthorized")
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	if strings.HasPrefix(cleanTarget, "/") {
		cleanTarget = strings.TrimPrefix(cleanTarget, "/")
	}

	fullPath := filepath.Join(p.workspaceDir, cleanTarget)
	rel, err := filepath.Rel(p.workspaceDir, fullPath)
	if err != nil {
		return "", err
	}

	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", ErrPathTraversal
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
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{baseStorageDir: filepath.Clean(baseStorageDir)}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", ErrUnauthorized
	}

	// Assuming OrganizationID is what we use to tenant-isolate.
	// If OrganizationID is missing, fallback to Subject?
	tenantID := claims.OrganizationID
	if tenantID == "" {
		tenantID = claims.Subject
	}

	if tenantID == "" {
		return "", errors.New("missing tenant identification in claims")
	}

	tenantDir := filepath.Join(p.baseStorageDir, tenantID)

	cleanTarget := filepath.Clean(target)
	if strings.HasPrefix(cleanTarget, "/") {
		cleanTarget = strings.TrimPrefix(cleanTarget, "/")
	}

	fullPath := filepath.Join(tenantDir, cleanTarget)
	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil {
		return "", err
	}

	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", ErrPathTraversal
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
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

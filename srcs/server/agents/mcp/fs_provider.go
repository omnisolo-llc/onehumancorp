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

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

type LocalFSProvider struct {
	rootDir string
}

func NewLocalFSProvider(rootDir string) *LocalFSProvider {
	return &LocalFSProvider{rootDir: filepath.Clean(rootDir)}
}

func (l *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	if filepath.IsAbs(reqPath) {
		return "", errors.New("absolute paths are not allowed")
	}
	cleanReq := filepath.Clean(reqPath)
	if strings.HasPrefix(cleanReq, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	fullPath := filepath.Join(l.rootDir, cleanReq)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != l.rootDir && !strings.HasPrefix(cleanPath, l.rootDir+string(filepath.Separator)) {
		return "", errors.New("path escapes root directory")
	}
	return cleanPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := l.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0600)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolved, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (c *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims")
	}

	if filepath.IsAbs(reqPath) {
		return "", errors.New("absolute paths are not allowed")
	}
	cleanReq := filepath.Clean(reqPath)
	if strings.HasPrefix(cleanReq, "..") {
		return "", errors.New("directory traversal not allowed")
	}

	tenantDir := filepath.Join(c.baseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanReq)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("path escapes tenant directory")
	}
	return cleanPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := c.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := c.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0600)
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	resolved, err := c.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(resolved)
}

type FSServer struct {
	provider FileSystemProvider
}

func NewHybridFSServer(isStandalone bool, baseDir string) *FSServer {
	if isStandalone {
		return &FSServer{provider: NewLocalFSProvider(baseDir)}
	}
	return &FSServer{provider: NewCloudFSProvider(baseDir)}
}

func (s *FSServer) Provider() FileSystemProvider {
	return s.provider
}

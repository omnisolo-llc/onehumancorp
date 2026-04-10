package mcp

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
	baseDir string
}

func (p *LocalFSProvider) sanitizePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths not allowed: %s", reqPath)
	}
	fullPath := filepath.Join(p.baseDir, cleanPath)
	rel, err := filepath.Rel(p.baseDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path escapes base directory: %s", reqPath)
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
	baseVolume string
}

func (p *CloudFSProvider) sanitizePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized or missing organization ID in context")
	}
	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths not allowed: %s", reqPath)
	}
	tenantDir := filepath.Join(p.baseVolume, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanPath)
	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path escapes tenant directory: %s", reqPath)
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

func NewFileSystemProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &CloudFSProvider{baseVolume: baseDir}
	}
	return &LocalFSProvider{baseDir: baseDir}
}

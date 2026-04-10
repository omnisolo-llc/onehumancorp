package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) securePath(requestedPath string) (string, error) {
	cleanPath := filepath.Clean(requestedPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}
	fullPath := filepath.Join(p.workspaceDir, cleanPath)

	// Prevent directory traversal
	relPath, err := filepath.Rel(p.workspaceDir, fullPath)
	if err != nil || strings.HasPrefix(relPath, "..") {
		return "", fmt.Errorf("path is outside workspace")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.securePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		names = append(names, name)
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant isolation
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) securePath(ctx context.Context, requestedPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", fmt.Errorf("unauthorized: missing claims")
	}

	tenantID := claims.OrganizationID
	if tenantID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}

	tenantDir := filepath.Join(p.baseDir, tenantID)

	cleanPath := filepath.Clean(requestedPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	fullPath := filepath.Join(tenantDir, cleanPath)

	// Prevent directory traversal outside tenant dir
	relPath, err := filepath.Rel(tenantDir, fullPath)
	if err != nil || strings.HasPrefix(relPath, "..") {
		return "", fmt.Errorf("path is outside tenant workspace")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		names = append(names, name)
	}
	return names, nil
}

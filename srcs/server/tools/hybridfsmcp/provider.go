package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for the local file system.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.baseDir, path))
	if !strings.HasPrefix(fullPath, p.baseDir) {
		return "", errors.New("directory traversal detected")
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
		return nil, err
	}
	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	// Simple implementation for searching files
	var results []string
	err := filepath.Walk(p.baseDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), query) {
			relPath, err := filepath.Rel(p.baseDir, path)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})
	return results, err
}

// CloudFSProvider implements FileSystemProvider for the cloud, scoping access by tenant.
// This is a simplified mock implementation for the sake of the exercise.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return "", errors.New("unauthorized: missing claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	fullPath := filepath.Clean(filepath.Join(tenantDir, path))
	if !strings.HasPrefix(fullPath, tenantDir) {
		return "", errors.New("directory traversal detected")
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
		return nil, err
	}
	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)

	var results []string
	err := filepath.Walk(tenantDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), query) {
			relPath, err := filepath.Rel(tenantDir, path)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})
	return results, err
}

// NewProviderFactory returns the appropriate provider based on the environment.
func NewProviderFactory(isCloud bool, baseDir string) FileSystemProvider {
	if isCloud {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

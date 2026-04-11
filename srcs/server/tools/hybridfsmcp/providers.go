package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) resolve(path string) (string, error) {
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	fullPath := filepath.Join(p.workspaceDir, cleanPath)
	if !strings.HasPrefix(fullPath, p.workspaceDir) {
		return "", fmt.Errorf("path traversal attempt: %s", path)
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolve(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolve(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolve(path)
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
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) resolve(path string) (string, error) {
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	fullPath := filepath.Join(p.baseDir, cleanPath)
	if !strings.HasPrefix(fullPath, p.baseDir) {
		return "", fmt.Errorf("path traversal attempt: %s", path)
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolve(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolve(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolve(path)
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
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func NewProviderFromEnv(workspaceDir string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(workspaceDir)
	}
	return NewLocalFSProvider(workspaceDir)
}

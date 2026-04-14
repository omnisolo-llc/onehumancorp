package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"github.com/onehumancorp/mono/srcs/server/utils"
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
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: workspaceDir}
}

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(p.workspaceDir, targetPath))
	if !strings.HasPrefix(fullPath, filepath.Clean(p.workspaceDir)+string(filepath.Separator)) && fullPath != filepath.Clean(p.workspaceDir) {
		return "", fmt.Errorf("path traversal denied")
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.securePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return utils.WriteFileAtomic(fullPath, data, 0644)
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
	names := []string{}
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	baseStorageDir string
}

func NewCloudFSProvider(baseStorageDir string) *CloudFSProvider {
	return &CloudFSProvider{baseStorageDir: baseStorageDir}
}

func (p *CloudFSProvider) securePath(ctx context.Context, targetPath string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", fmt.Errorf("unauthorized: missing organization claims")
	}

	tenantDir := filepath.Join(p.baseStorageDir, orgID)
	fullPath := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if !strings.HasPrefix(fullPath, filepath.Clean(tenantDir)+string(filepath.Separator)) && fullPath != filepath.Clean(tenantDir) {
		return "", fmt.Errorf("path traversal denied")
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

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return utils.WriteFileAtomic(fullPath, data, 0644)
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
	names := []string{}
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

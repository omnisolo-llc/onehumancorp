package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements mcp.FileSystemProvider for standalone environments.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) validatePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, path))
	if cleanPath == p.baseDir || strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", fmt.Errorf("path access denied: %s", path)
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte, perm fs.FileMode) error {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	// Ensure parent directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, perm)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	dirEntries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var fileInfos []fs.FileInfo
	for _, entry := range dirEntries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip if info cannot be retrieved
		}
		fileInfos = append(fileInfos, info)
	}
	return fileInfos, nil
}

// CloudFSProvider implements mcp.FileSystemProvider for cloud-native environments.
// It scopes access by the tenant's organization ID from the context.
type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) validatePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization claims in context")
	}

	// Scope strictly to baseDir / organizationID
	tenantBase := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantBase, path))

	if cleanPath == tenantBase || strings.HasPrefix(cleanPath, tenantBase+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", fmt.Errorf("path access denied: %s", path)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte, perm fs.FileMode) error {
	fullPath, err := p.validatePath(ctx, path)
	if err != nil {
		return err
	}
	// Ensure parent directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, perm)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	dirEntries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}
	var fileInfos []fs.FileInfo
	for _, entry := range dirEntries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		fileInfos = append(fileInfos, info)
	}
	return fileInfos, nil
}

var _ mcp.FileSystemProvider = (*LocalFSProvider)(nil)
var _ mcp.FileSystemProvider = (*CloudFSProvider)(nil)

package hybridfsmcp

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
	ListDir(ctx context.Context, path string) ([]DirEntry, error)
}

type DirEntry struct {
	Name  string
	IsDir bool
}

type LocalFSProvider struct {
	Workspace string
}

func NewLocalFSProvider(workspace string) *LocalFSProvider {
	return &LocalFSProvider{Workspace: workspace}
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	absWorkspace, err := filepath.Abs(p.Workspace)
	if err != nil {
		return "", err
	}

	// Ensure the workspace path ends with a separator so we don't fall for prefix bypasses (e.g., /tmp/workspace-secrets)
	if !strings.HasSuffix(absWorkspace, string(filepath.Separator)) {
		absWorkspace += string(filepath.Separator)
	}

	targetPath := filepath.Join(absWorkspace, path)
	absTarget, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	// When comparing, also ensure absTarget acts within the directory structure
	if !strings.HasPrefix(absTarget, absWorkspace) && absTarget != filepath.Clean(p.Workspace) {
		return "", fmt.Errorf("path escapes workspace")
	}
	return absTarget, nil
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
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]DirEntry, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	result := make([]DirEntry, len(entries))
	for i, entry := range entries {
		result[i] = DirEntry{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
		}
	}
	return result, nil
}

type CloudFSProvider struct {
	BaseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseDir: baseDir}
}

func (p *CloudFSProvider) getTenantID(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		// fallback to test key for tests
		claims, _ = ctx.Value(auth.ClaimsContextKeyForTest).(*auth.Claims)
		if claims == nil {
			return "", fmt.Errorf("missing auth claims in context")
		}
	}
	if claims.OrganizationID == "" {
		return "", fmt.Errorf("tenant ID (OrganizationID) missing from auth claims")
	}
	return claims.OrganizationID, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, path string) (string, error) {
	tenantID, err := p.getTenantID(ctx)
	if err != nil {
		return "", err
	}

	absBase, err := filepath.Abs(filepath.Join(p.BaseDir, tenantID))
	if err != nil {
		return "", err
	}

	// Ensure tenant directory exists
	if err := os.MkdirAll(absBase, 0755); err != nil {
		return "", err
	}

	if !strings.HasSuffix(absBase, string(filepath.Separator)) {
		absBase += string(filepath.Separator)
	}

	targetPath := filepath.Join(absBase, path)
	absTarget, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	if !strings.HasPrefix(absTarget, absBase) && absTarget != filepath.Clean(filepath.Join(p.BaseDir, tenantID)) {
		return "", fmt.Errorf("path escapes tenant workspace")
	}
	return absTarget, nil
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
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]DirEntry, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return []DirEntry{}, nil
		}
		return nil, err
	}

	result := make([]DirEntry, len(entries))
	for i, entry := range entries {
		result[i] = DirEntry{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
		}
	}
	return result, nil
}

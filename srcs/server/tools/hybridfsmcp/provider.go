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
	ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
	workspaceRoot string
}

func NewLocalFSProvider(root string) *LocalFSProvider {
	abs, _ := filepath.Abs(root)
	return &LocalFSProvider{workspaceRoot: filepath.Clean(abs)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		cleanTarget = strings.TrimPrefix(cleanTarget, "/")
	}
	absPath := filepath.Join(p.workspaceRoot, cleanTarget)
	absPath = filepath.Clean(absPath)

	if !strings.HasPrefix(absPath, p.workspaceRoot+string(filepath.Separator)) && absPath != p.workspaceRoot {
		return "", fmt.Errorf("path access denied")
	}
	return absPath, nil
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
	os.MkdirAll(filepath.Dir(fullPath), 0755)
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
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	storageRoot string
}

func NewCloudFSProvider(root string) *CloudFSProvider {
	abs, _ := filepath.Abs(root)
	return &CloudFSProvider{storageRoot: filepath.Clean(abs)}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization ID")
	}

	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		cleanTarget = strings.TrimPrefix(cleanTarget, "/")
	}

	tenantScopedPath := filepath.Join(claims.OrganizationID, cleanTarget)

	absPath := filepath.Join(p.storageRoot, tenantScopedPath)
	absPath = filepath.Clean(absPath)

	expectedPrefix := filepath.Join(p.storageRoot, claims.OrganizationID)
	if !strings.HasPrefix(absPath, expectedPrefix+string(filepath.Separator)) && absPath != expectedPrefix {
		return "", fmt.Errorf("path access denied")
	}

	return absPath, nil
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
	os.MkdirAll(filepath.Dir(fullPath), 0755)
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
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

func NewProvider(root string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(root)
	}
	return NewCloudFSProvider(root)
}

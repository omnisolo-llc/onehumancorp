package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts the file system operations for the MCP server.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	rootDir string
}

// CloudFSProvider implements FileSystemProvider for Cloud-Native mode.
type CloudFSProvider struct {
	rootDir string
}

// NewFileSystemProvider creates the appropriate FileSystemProvider based on mode.
func NewFileSystemProvider(isStandalone bool, root string) FileSystemProvider {
	if root == "" {
		root = os.Getenv("OHC_FS_ROOT")
		if root == "" {
			root = "/tmp/ohc_workspace"
		}
	}

	if isStandalone {
		return &LocalFSProvider{rootDir: root}
	}
	return &CloudFSProvider{rootDir: root}
}

// safeJoin securely joins paths to prevent path traversal vulnerabilities.
func safeJoin(base string, target string) (string, error) {
	cleanTarget := filepath.Clean("/" + target)
	cleanTarget = strings.TrimPrefix(cleanTarget, "/")

	finalPath := filepath.Join(base, cleanTarget)
	if finalPath == base || strings.HasPrefix(finalPath, base+string(filepath.Separator)) {
		return finalPath, nil
	}
	return "", errors.New("path traversal detected")
}

// LocalFSProvider Methods
func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	return safeJoin(p.rootDir, path)
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		if entry.IsDir() {
			results = append(results, entry.Name()+"/")
		} else {
			results = append(results, entry.Name())
		}
	}
	return results, nil
}

// CloudFSProvider Methods
func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	tenantBase := filepath.Join(p.rootDir, claims.OrganizationID)
	return safeJoin(tenantBase, path)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, entry := range entries {
		if entry.IsDir() {
			results = append(results, entry.Name()+"/")
		} else {
			results = append(results, entry.Name())
		}
	}
	return results, nil
}

package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct{}

func (p *CloudFSProvider) getBasePath(claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}
	root := os.Getenv("OHC_FS_ROOT")
	if root == "" {
		root = os.TempDir()
	}
	return filepath.Join(filepath.Clean(root), "tenants", claims.OrganizationID), nil
}

func (p *CloudFSProvider) resolveAndValidatePath(base string, requestedPath string) (string, error) {
	target := filepath.Join(base, filepath.Clean(requestedPath))
	if target == base || strings.HasPrefix(target, base+string(filepath.Separator)) {
		return target, nil
	}
	return "", fmt.Errorf("path traversal denied")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	basePath, err := p.getBasePath(claims)
	if err != nil {
		return nil, err
	}
	targetPath, err := p.resolveAndValidatePath(basePath, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(targetPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	basePath, err := p.getBasePath(claims)
	if err != nil {
		return err
	}
	targetPath, err := p.resolveAndValidatePath(basePath, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(targetPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(targetPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	basePath, err := p.getBasePath(claims)
	if err != nil {
		return nil, err
	}
	targetPath, err := p.resolveAndValidatePath(basePath, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(targetPath)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, entry := range entries {
		res = append(res, entry.Name())
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	basePath, err := p.getBasePath(claims)
	if err != nil {
		return nil, err
	}
	targetPath, err := p.resolveAndValidatePath(basePath, path)
	if err != nil {
		return nil, err
	}
	var res []string
	err = filepath.WalkDir(targetPath, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return nil // skip errors
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(targetPath, path)
			res = append(res, rel)
		}
		return nil
	})
	return res, err
}

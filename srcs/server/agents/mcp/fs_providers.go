package mcp

import (
	"context"
	"errors"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

var (
	ErrPathEscape = errors.New("path escapes base directory")
	ErrNoAuth     = errors.New("no auth claims found in context for cloud access")
)

type LocalFSProvider struct {
	baseDir string
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(reqPath)
	fullPath := filepath.Join(p.baseDir, cleanPath)
	rel, err := filepath.Rel(p.baseDir, fullPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", ErrPathEscape
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return ioutil.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(resolved), 0755)
	if err != nil {
		return err
	}
	return ioutil.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := ioutil.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

type CloudFSProvider struct {
	globalBaseDir string
}

func (p *CloudFSProvider) getTenantBaseDir(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", ErrNoAuth
	}
	return filepath.Join(p.globalBaseDir, claims.OrganizationID), nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	tenantDir, err := p.getTenantBaseDir(ctx)
	if err != nil {
		return "", err
	}
	cleanPath := filepath.Clean(reqPath)
	fullPath := filepath.Join(tenantDir, cleanPath)
	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", ErrPathEscape
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return ioutil.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(resolved), 0755)
	if err != nil {
		return err
	}
	return ioutil.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := ioutil.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func NewFileSystemProvider(ctx context.Context, workspaceDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return &LocalFSProvider{baseDir: workspaceDir}
	}
	return &CloudFSProvider{globalBaseDir: workspaceDir}
}

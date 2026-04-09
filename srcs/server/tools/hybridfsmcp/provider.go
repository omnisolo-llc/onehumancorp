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
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, path, pattern string) ([]string, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider() *LocalFSProvider {
	baseDir := os.Getenv("OHC_WORKSPACE_DIR")
	if baseDir == "" {
		baseDir = "/tmp/ohc_workspace"
	}
	os.MkdirAll(baseDir, 0700)
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) maskError(err error) error {
	if err == nil {
		return nil
	}
	msg := err.Error()
	if strings.Contains(msg, p.baseDir) {
		return fmt.Errorf("%s", strings.ReplaceAll(msg, p.baseDir, "<WORKSPACE>"))
	}
	return err
}
func (p *LocalFSProvider) securePath(requestedPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, requestedPath))
	if !(strings.HasPrefix(cleanPath, p.baseDir+"/") || cleanPath == p.baseDir) {
		return "", fmt.Errorf("path access denied: %s", requestedPath)
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	data, err := os.ReadFile(safePath)
	return data, p.maskError(err)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	safePath, err := p.securePath(path)
	if err != nil {
		return err
	}
	os.MkdirAll(filepath.Dir(safePath), 0700)
	err = os.WriteFile(safePath, content, 0600)
	return p.maskError(err)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, p.maskError(err)
	}
	var res []string
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		if e.IsDir() {
			res = append(res, fmt.Sprintf("%s/ (dir)", e.Name()))
		} else {
			res = append(res, fmt.Sprintf("%s (file, %d bytes)", e.Name(), info.Size()))
		}
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path, pattern string) ([]string, error) {
	safePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.Walk(safePath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), pattern) {

			matches = append(matches, path)
		}
		return nil
	})
	if err != nil {
		return nil, p.maskError(err)
	}
	// Make paths relative to baseDir for output
	var relMatches []string
	for _, m := range matches {
		rel, err := filepath.Rel(p.baseDir, m)
		if err == nil {
			relMatches = append(relMatches, rel)
		}
	}
	return relMatches, nil
}

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider() *CloudFSProvider {
	baseDir := os.Getenv("OHC_TENANT_PV_DIR")
	if baseDir == "" {
		baseDir = "/tmp/ohc_cloud_pv"
	}
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) maskError(claims *auth.Claims, err error) error {
	if err == nil {
		return nil
	}
	msg := err.Error()
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	if strings.Contains(msg, tenantDir) {
		return fmt.Errorf("%s", strings.ReplaceAll(msg, tenantDir, "<TENANT_PV>"))
	}
	return err
}
func (p *CloudFSProvider) securePath(claims *auth.Claims, requestedPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	os.MkdirAll(tenantDir, 0700)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, requestedPath))
	if !(strings.HasPrefix(cleanPath, tenantDir+"/") || cleanPath == tenantDir) {
		return "", fmt.Errorf("path access denied: %s", requestedPath)
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}
	data, err := os.ReadFile(safePath)
	return data, p.maskError(claims, err)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	safePath, err := p.securePath(claims, path)
	if err != nil {
		return err
	}
	os.MkdirAll(filepath.Dir(safePath), 0700)
	err = os.WriteFile(safePath, content, 0600)
	return p.maskError(claims, err)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, p.maskError(claims, err)
	}
	var res []string
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		if e.IsDir() {
			res = append(res, fmt.Sprintf("%s/ (dir)", e.Name()))
		} else {
			res = append(res, fmt.Sprintf("%s (file, %d bytes)", e.Name(), info.Size()))
		}
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path, pattern string) ([]string, error) {
	safePath, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.Walk(safePath, func(filePath string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), pattern) {
			matches = append(matches, filePath)
		}
		return nil
	})
	if err != nil {
		return nil, p.maskError(claims, err)
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	var relMatches []string
	for _, m := range matches {
		rel, err := filepath.Rel(tenantDir, m)
		if err == nil {
			relMatches = append(relMatches, rel)
		}
	}
	return relMatches, nil
}

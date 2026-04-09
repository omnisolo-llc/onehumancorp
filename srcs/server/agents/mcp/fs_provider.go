package mcp

import (
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

var (
	ErrAccessDenied = errors.New("access denied: path outside allowed directory")
	ErrInvalidPath  = errors.New("invalid path format")
)

type FileSystemProvider interface {
	ReadFile(path string) ([]byte, error)
	WriteFile(path string, content []byte) error
	ListDir(path string) ([]string, error)
}

type LocalFSProvider struct {
	WorkspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{WorkspaceDir: workspaceDir}
}

func (p *LocalFSProvider) validatePath(reqPath string) (string, error) {
	if filepath.IsAbs(reqPath) {
		return "", ErrInvalidPath
	}
	cleanReq := filepath.Clean(reqPath)
	fullPath := filepath.Join(p.WorkspaceDir, cleanReq)

	absWorkspace, err := filepath.Abs(p.WorkspaceDir)
	if err != nil {
		return "", err
	}
	absFull, err := filepath.Abs(fullPath)
	if err != nil {
		return "", err
	}

	if !strings.HasPrefix(absFull, absWorkspace) || (absFull != absWorkspace && !strings.HasPrefix(absFull, absWorkspace+string(filepath.Separator))) {
		return "", ErrAccessDenied
	}
	return absFull, nil
}

func (p *LocalFSProvider) ReadFile(path string) ([]byte, error) {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(path string, content []byte) error {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(fullPath), 0700)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0600)
}

func (p *LocalFSProvider) ListDir(path string) ([]string, error) {
	fullPath, err := p.validatePath(path)
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
	BaseDir string
	TenantID string
}

func NewCloudFSProvider(baseDir string, claims *auth.Claims) *CloudFSProvider {
	return &CloudFSProvider{
		BaseDir: baseDir,
		TenantID: claims.OrganizationID,
	}
}

func (p *CloudFSProvider) validatePath(reqPath string) (string, error) {
	if filepath.IsAbs(reqPath) {
		return "", ErrInvalidPath
	}
	cleanReq := filepath.Clean(reqPath)
	tenantPath := filepath.Join(p.BaseDir, p.TenantID)
	fullPath := filepath.Join(tenantPath, cleanReq)

	absTenant, err := filepath.Abs(tenantPath)
	if err != nil {
		return "", err
	}
	absFull, err := filepath.Abs(fullPath)
	if err != nil {
		return "", err
	}

	if !strings.HasPrefix(absFull, absTenant) || (absFull != absTenant && !strings.HasPrefix(absFull, absTenant+string(filepath.Separator))) {
		return "", ErrAccessDenied
	}
	return absFull, nil
}

func (p *CloudFSProvider) ReadFile(path string) ([]byte, error) {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(path string, content []byte) error {
	fullPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(fullPath), 0700)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0600)
}

func (p *CloudFSProvider) ListDir(path string) ([]string, error) {
	fullPath, err := p.validatePath(path)
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

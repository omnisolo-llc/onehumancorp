package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	abs, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: abs}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("absolute paths not allowed")
	}
	fullPath := filepath.Join(p.workspaceDir, cleanTarget)

	if !strings.HasPrefix(fullPath, p.workspaceDir) || (len(fullPath) > len(p.workspaceDir) && fullPath[len(p.workspaceDir)] != filepath.Separator) {
		if fullPath != p.workspaceDir {
			return "", fmt.Errorf("path escapes workspace boundary")
		}
	}
	return fullPath, nil
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
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0600)
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
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

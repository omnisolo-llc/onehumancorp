package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	abs, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(abs, 0700); err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: abs}, nil
}

func (p *LocalFSProvider) resolvePath(key string) (string, error) {
	cleanKey := filepath.Clean(key)
	if filepath.IsAbs(cleanKey) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}
	fullPath := filepath.Join(p.workspaceDir, cleanKey)
	if !strings.HasPrefix(fullPath, p.workspaceDir+string(filepath.Separator)) && fullPath != p.workspaceDir {
		return "", fmt.Errorf("path escapes workspace boundary")
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, key string) ([]byte, error) {
	path, err := p.resolvePath(key)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(path)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, key string, content []byte) error {
	path, err := p.resolvePath(key)
	if err != nil {
		return err
	}
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(path, content, 0600)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, prefix string) ([]string, error) {
	path, err := p.resolvePath(prefix)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	// Simple implementation for searching files
	var res []string
	err := filepath.Walk(p.workspaceDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), pattern) {
			rel, err := filepath.Rel(p.workspaceDir, path)
			if err == nil {
				res = append(res, rel)
			}
		}
		return nil
	})
	return res, err
}

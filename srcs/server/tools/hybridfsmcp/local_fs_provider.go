package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	workspaceRoot string
}

func NewLocalFSProvider(workspaceRoot string) *LocalFSProvider {
	provider := &LocalFSProvider{workspaceRoot: filepath.Clean(workspaceRoot)}
	if !filepath.IsAbs(provider.workspaceRoot) {
		absRoot, err := filepath.Abs(provider.workspaceRoot)
		if err == nil {
			provider.workspaceRoot = filepath.Clean(absRoot)
		}
	}
	return provider
}

func (p *LocalFSProvider) validatePath(targetPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.workspaceRoot, targetPath))
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(filepath.Clean(absPath), p.workspaceRoot+string(filepath.Separator)) && filepath.Clean(absPath) != p.workspaceRoot {
		return "", fmt.Errorf("path access denied: outside workspace bounds")
	}
	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	validPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(validPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	validPath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(validPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(validPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	validPath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(validPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

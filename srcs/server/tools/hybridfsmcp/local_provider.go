package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	BaseDir string
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.BaseDir, reqPath))

	absBase, err := filepath.Abs(p.BaseDir)
	if err != nil {
		return "", err
	}

	absClean, err := filepath.Abs(cleanPath)
	if err != nil {
		return "", err
	}

	if !strings.HasPrefix(absClean, absBase+string(filepath.Separator)) && absClean != absBase {
		return "", fmt.Errorf("path escapes base directory: %s", reqPath)
	}

	return absClean, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

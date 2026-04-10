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

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (l *LocalFSProvider) securePath(path string) (string, error) {
	fullPath := filepath.Clean(filepath.Join(l.workspaceDir, path))
	if !strings.HasPrefix(fullPath, l.workspaceDir+string(os.PathSeparator)) && fullPath != l.workspaceDir {
		return "", fmt.Errorf("path traversal attempt: %s", path)
	}
	return fullPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := l.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := l.securePath(path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := l.securePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var res []string
	for _, entry := range entries {
		res = append(res, entry.Name())
	}
	return res, nil
}

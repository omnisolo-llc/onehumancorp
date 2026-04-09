package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider maps directly to the local file system with safety bounds.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{
		workspaceDir: absWorkspace,
	}, nil
}

func (l *LocalFSProvider) securePath(reqPath string) (string, error) {
	targetPath := filepath.Join(l.workspaceDir, reqPath)
	var evalErr error
	targetPath, evalErr = filepath.EvalSymlinks(targetPath)
	if evalErr != nil {
		targetPath = filepath.Join(l.workspaceDir, reqPath)
	}
	absPath, err := filepath.Abs(targetPath)
	if err != nil {
		return "", err
	}

	// Ensure directory traversal does not escape the workspace.
	// We check for exact match or prefix + separator to allow clean paths
	cleanWorkspace := filepath.Clean(l.workspaceDir)
	if absPath != cleanWorkspace && !strings.HasPrefix(absPath, cleanWorkspace+string(filepath.Separator)) {
		return "", fmt.Errorf("access denied: path escapes workspace")
	}
	return absPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := l.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := l.securePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := l.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

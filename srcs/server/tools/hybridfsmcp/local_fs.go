package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for Standalone mode,
// bounding access to a specified workspace directory.
type LocalFSProvider struct {
	workspace string
}

func NewLocalFSProvider(workspace string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspace)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspace: absWorkspace}, nil
}

// resolvePath ensures the target path is within the workspace directory.
func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	fullPath := filepath.Join(p.workspace, target)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != p.workspace && !strings.HasPrefix(cleanPath, p.workspace+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes workspace boundary")
	}

	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	securePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(securePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	securePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(securePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(securePath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	securePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(securePath)
	if err != nil {
		return nil, err
	}

	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

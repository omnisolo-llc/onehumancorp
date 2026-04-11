package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for the local filesystem.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: filepath.Clean(baseDir),
	}
}

// ReadFile reads a file from the local filesystem, ensuring it is within the baseDir.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath := filepath.Join(p.baseDir, path)
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return nil, errors.New("path traversal detected")
	}

	return os.ReadFile(fullPath)
}

// WriteFile writes a file to the local filesystem, ensuring it is within the baseDir.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath := filepath.Join(p.baseDir, path)
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return errors.New("path traversal detected")
	}

	if filepath.Clean(path) == "." || filepath.Clean(path) == "/" {
		return errors.New("cannot write to root directory")
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

// ListDir lists the contents of a directory on the local filesystem, ensuring it is within the baseDir.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath := filepath.Join(p.baseDir, path)
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return nil, errors.New("path traversal detected")
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

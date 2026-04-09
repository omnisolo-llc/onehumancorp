package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider by mapping requests to the local
// host filesystem, bounded to a specific base directory to prevent path traversal.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{
		baseDir: absBase,
	}, nil
}

// resolvePath resolves the given path against the base directory and ensures
// that it does not escape the base directory.
func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	// Prevent absolute paths from escaping
	if filepath.IsAbs(targetPath) {
		return "", errors.New("path traversal detected")
	}

	cleanTarget := filepath.Clean(targetPath)

	fullPath := filepath.Join(p.baseDir, cleanTarget)

	// Clean the resulting path again to resolve any . or ..
	fullPath = filepath.Clean(fullPath)

	if !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) && fullPath != p.baseDir {
		return "", errors.New("path traversal detected")
	}

	return fullPath, nil
}

// ReadFile reads the contents of the file at the specified path.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	return os.ReadFile(fullPath)
}

// WriteFile writes the given data to the file at the specified path.
// It creates the parent directories if they do not exist.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

// ListDir lists the entries of the directory at the specified path.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
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
		name := entry.Name()
		if entry.IsDir() {
			name += "/"
		}
		names = append(names, name)
	}

	return names, nil
}

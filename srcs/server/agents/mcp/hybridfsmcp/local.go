package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for the local filesystem.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the given base directory.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.baseDir, path))
	if err != nil {
		return "", err
	}
	rel, err := filepath.Rel(p.baseDir, absPath)
	if err != nil || strings.HasPrefix(rel, "..") || rel == ".." {
		return "", errors.New("access denied: path escapes base directory")
	}
	return absPath, nil
}

// ReadFile reads a file from the local filesystem.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

// WriteFile writes data to a file on the local filesystem.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte, perm fs.FileMode) error {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	return os.WriteFile(absPath, data, perm)
}

// ListDir lists the contents of a directory on the local filesystem.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(absPath)
}

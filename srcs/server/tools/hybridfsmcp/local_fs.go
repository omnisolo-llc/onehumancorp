package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: filepath.Clean(baseDir)}
}

func (l *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	if filepath.IsAbs(targetPath) {
		return "", errors.New("absolute paths are not allowed")
	}
	cleanTarget := filepath.Clean(targetPath)
	fullPath := filepath.Join(l.BaseDir, cleanTarget)
	if !strings.HasPrefix(fullPath, l.BaseDir+string(filepath.Separator)) && fullPath != l.BaseDir {
		return "", errors.New("path traversal detected")
	}
	return fullPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return err
	}
	err = os.MkdirAll(filepath.Dir(fullPath), 0755)
	if err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}
	return os.WriteFile(fullPath, content, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(fullPath)
}

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

// LocalFSProvider implements FileSystemProvider for a local filesystem bounded to a workspace directory.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		// Fallback to exactly what was provided
		absBase = baseDir
	}
	return &LocalFSProvider{baseDir: filepath.Clean(absBase)}
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Join(p.baseDir, path)
	cleanPath := filepath.Clean(fullPath)

	// Prevent path traversal
	if !strings.HasPrefix(cleanPath, p.baseDir+string(os.PathSeparator)) && cleanPath != p.baseDir {
		return "", errors.New("access denied: path escapes workspace boundary")
	}
	return cleanPath, nil
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
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries where info can't be read
		}
		infos = append(infos, info)
	}

	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, directory string, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(directory)
	if err != nil {
		return nil, err
	}

	var results []string
	err = filepath.Walk(resolved, func(path string, info fs.FileInfo, err error) error {
		if err != nil {
			return nil // Skip errors accessing specific files
		}

		if !info.IsDir() {
			matched, err := filepath.Match(pattern, info.Name())
			if err == nil && matched {
				relPath, err := filepath.Rel(p.baseDir, path)
				if err == nil {
					results = append(results, relPath)
				}
			}
		}
		return nil
	})

	return results, err
}

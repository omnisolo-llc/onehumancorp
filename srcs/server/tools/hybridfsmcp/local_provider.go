package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for the local filesystem.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a new LocalFSProvider with the given base directory.
func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(absPath, 0755); err != nil {
		return nil, err
	}
	return &LocalFSProvider{basePath: absPath}, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// resolvePath checks and resolves the given path against the basePath to prevent directory traversal.
func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	fullPath := filepath.Join(p.basePath, path)
	fullPath = filepath.Clean(fullPath)

	// Ensure that the resulting path is still within basePath
	// Appending Separator to basePath ensures we don't match partial directory names.
	baseDirWithSep := p.basePath
	if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
		baseDirWithSep += string(filepath.Separator)
	}

	// If it's exactly the base path, it's safe. Otherwise it must start with the separated prefix.
	if fullPath != p.basePath && !strings.HasPrefix(fullPath, baseDirWithSep) {
		return "", fmt.Errorf("path escapes base directory: %s", path)
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure the parent directory exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, data, 0644)
}

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
		names = append(names, entry.Name())
	}

	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(fullPath, func(currPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}

		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}

		if matched {
			relPath, err := filepath.Rel(p.basePath, currPath)
			if err == nil {
				matches = append(matches, filepath.ToSlash(relPath))
			}
		}
		return nil
	})

	return matches, err
}

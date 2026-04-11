package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for a local directory.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

// resolvePath resolves a given path relative to the base directory and ensures
// it does not escape the base directory (path traversal protection).
func (p *LocalFSProvider) resolvePath(path string) (string, error) {
    // Trim leading slashes to prevent filepath.Join from treating it as absolute
    path = strings.TrimPrefix(path, "/")
	fullPath := filepath.Clean(filepath.Join(p.baseDir, path))

	// Basic path traversal protection
	prefix := p.baseDir
	if !strings.HasSuffix(prefix, string(filepath.Separator)) {
		prefix += string(filepath.Separator)
	}
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, prefix) {
		return "", errors.New("access denied: path escapes base directory")
	}

	return fullPath, nil
}

// ReadFile reads the content of a file.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

// WriteFile writes content to a file, creating necessary directories.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

// ListDir lists files and directories under a given path.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]map[string]interface{}, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // skip entries we can't stat
		}
		results = append(results, map[string]interface{}{
			"name":  entry.Name(),
			"isDir": entry.IsDir(),
			"size":  info.Size(),
		})
	}

	return results, nil
}

// IsLocal returns true for local providers.
func (p *LocalFSProvider) IsLocal() bool {
	return true
}

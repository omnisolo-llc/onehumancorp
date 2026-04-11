package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

var (
	ErrAccessDenied = errors.New("access denied: path escapes workspace")
)

// LocalFSProvider implements FileSystemProvider for the local file system.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider with the given base workspace directory.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{
		baseDir: absBaseDir + string(filepath.Separator),
	}, nil
}

// resolvePath returns the absolute path and ensures it does not escape the base directory.
func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	// If reqPath is absolute, we need to treat it as relative to baseDir to prevent escaping.
	// By stripping leading slash, filepath.Join will correctly append it to baseDir.
	cleanReq := filepath.Clean(reqPath)
	cleanReq = strings.TrimPrefix(cleanReq, "/")

	fullPath := filepath.Join(p.baseDir, cleanReq)
	absPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", err
	}

	// Add trailing separator to absPath if it's a directory to safely check prefix,
	// but for files we can't always do that easily without checking os.Stat.
	// Actually, just checking if absPath starts with p.baseDir (which has trailing sep)
	// OR is exactly the base directory (without trailing sep) is safer.
	baseDirNoSep := strings.TrimSuffix(p.baseDir, string(filepath.Separator))
	if !strings.HasPrefix(absPath, p.baseDir) && absPath != baseDirNoSep {
		return "", ErrAccessDenied
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure parent directories exist
	dir := filepath.Dir(absPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(absPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(absPath)
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
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(absPath, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			relPath, err := filepath.Rel(p.baseDir, path)
			if err == nil {
				matches = append(matches, relPath)
			}
		}
		return nil
	})

	if err != nil {
		return nil, err
	}
	return matches, nil
}

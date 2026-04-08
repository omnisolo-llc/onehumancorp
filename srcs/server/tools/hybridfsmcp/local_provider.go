package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	absPath := filepath.Join(p.baseDir, reqPath)
	rel, err := filepath.Rel(p.baseDir, absPath)
	if err != nil {
		return "", err
	}

	relSlash := filepath.ToSlash(rel)
	if relSlash == ".." || strings.HasPrefix(relSlash, "../") {
		return "", errors.New("directory traversal detected")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, tenantID, reqPath string) ([]byte, error) {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, tenantID, reqPath string, data []byte) error {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, tenantID, reqPath string) ([]string, error) {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, tenantID, reqPath, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(resolved, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() {
			matched, err := filepath.Match(pattern, d.Name())
			if err != nil {
				return err
			}
			if matched {
				relPath, err := filepath.Rel(p.baseDir, path)
				if err == nil {
					matches = append(matches, filepath.ToSlash(relPath))
				}
			}
		}
		return nil
	})

	if err != nil {
		return nil, err
	}

	return matches, nil
}

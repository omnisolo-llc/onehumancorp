package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		absBase = baseDir // fallback
	}
	return &LocalFSProvider{
		baseDir: absBase,
	}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absTarget, err := filepath.Abs(filepath.Join(p.baseDir, target))
	if err != nil {
		return "", err
	}

	// Prevent path traversal vulnerabilities where similar directory names overlap (e.g., tenant10 matching tenant1)
	if absTarget != p.baseDir && !strings.HasPrefix(absTarget, p.baseDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal attempt: %s", target)
	}
	return absTarget, nil
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
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var files []string
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if os.IsNotExist(err) {
			return files, nil
		}
		return nil, err
	}
	for _, e := range entries {
		if e.IsDir() {
			files = append(files, e.Name()+"/")
		} else {
			files = append(files, e.Name())
		}
	}
	return files, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var results []string
	err = filepath.WalkDir(fullPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(fullPath, path)
			results = append(results, rel)
		}
		return nil
	})
	return results, err
}

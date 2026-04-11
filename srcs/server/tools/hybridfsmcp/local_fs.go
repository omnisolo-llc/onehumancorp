package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		absBase = baseDir
	}
	return &LocalFSProvider{
		baseDir: absBase,
	}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	fullPath := filepath.Join(p.baseDir, target)
	fullPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", err
	}

	// Boundary check to prevent path traversal
	if fullPath != p.baseDir && !strings.HasPrefix(fullPath, p.baseDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path bounds violation: %s", target)
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, FileInfo{
			Name:    entry.Name(),
			IsDir:   entry.IsDir(),
			Size:    info.Size(),
			ModTime: info.ModTime().Format(time.RFC3339),
		})
	}
	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	var matches []string

	err := filepath.WalkDir(p.baseDir, func(path string, d os.DirEntry, err error) error {
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

	return matches, err
}

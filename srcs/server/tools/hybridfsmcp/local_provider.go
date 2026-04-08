package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		absPath = basePath // fallback
	}
	return &LocalFSProvider{basePath: absPath}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	fullPath := filepath.Join(p.basePath, target)
	cleanPath := filepath.Clean(fullPath)

	rel, err := filepath.Rel(p.basePath, cleanPath)
	if err != nil {
		return "", fmt.Errorf("path out of bounds")
	}
	if strings.HasPrefix(rel, "..") || rel == ".." {
		return "", fmt.Errorf("path out of bounds")
	}

	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var res []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		size := int64(0)
		if err == nil {
			size = info.Size()
		}
		res = append(res, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  size,
		})
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]FileInfo, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	var res []FileInfo
	err = filepath.Walk(safePath, func(walkPath string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), pattern) {
			res = append(res, FileInfo{
				Name:  info.Name(),
				IsDir: info.IsDir(),
				Size:  info.Size(),
			})
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	return res, nil
}

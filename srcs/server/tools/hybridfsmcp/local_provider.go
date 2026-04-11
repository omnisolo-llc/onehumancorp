package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absBase, _ := filepath.Abs(baseDir)
	return &LocalFSProvider{baseDir: absBase}
}

func (p *LocalFSProvider) validatePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.baseDir, target))
	if cleanTarget != p.baseDir && !strings.HasPrefix(cleanTarget, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("path traversal detected")
	}
	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var res []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		res = append(res, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

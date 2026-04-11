package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{BaseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanReq := strings.TrimPrefix(filepath.Clean(reqPath), "/")
	fullPath := filepath.Join(p.BaseDir, cleanReq)

	// Add trailing separator to baseDir for exact boundary checks unless baseDir is exact match
	checkBase := p.BaseDir
	if !strings.HasSuffix(checkBase, string(filepath.Separator)) {
		checkBase += string(filepath.Separator)
	}

	if fullPath != p.BaseDir && !strings.HasPrefix(fullPath, checkBase) {
		return "", fmt.Errorf("path traversal denied")
	}

	return fullPath, nil
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
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
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

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}

	return infos, nil
}

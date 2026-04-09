package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for local Standalone mode.
// It bounds access to a specific base directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	cleanBase := filepath.Clean(baseDir)
	if !filepath.IsAbs(cleanBase) {
		return nil, fmt.Errorf("base directory must be absolute")
	}
	return &LocalFSProvider{baseDir: cleanBase}, nil
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanPath := filepath.Clean(reqPath)
	if filepath.IsAbs(cleanPath) {
		return "", fmt.Errorf("path must be relative")
	}

	fullPath := filepath.Join(p.baseDir, cleanPath)

	// Ensure the resolved path is within baseDir to prevent directory traversal
	basePrefix := p.baseDir
	if !strings.HasSuffix(basePrefix, string(filepath.Separator)) {
		basePrefix += string(filepath.Separator)
	}

	if !strings.HasPrefix(fullPath, basePrefix) && fullPath != p.baseDir {
		return "", fmt.Errorf("path escapes base directory")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) (string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return "", err
	}

	data, err := os.ReadFile(fullPath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content string) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, []byte(content), 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

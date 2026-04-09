package mcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for the local file system.
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

func (p *LocalFSProvider) getLocalPath(reqPath string) (string, error) {
	cleanPath := filepath.Clean("/" + reqPath)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	fullPath := filepath.Join(p.basePath, cleanPath)

	// Ensure no directory traversal escapes basePath
	if !strings.HasPrefix(fullPath, p.basePath) {
		return "", fmt.Errorf("invalid path: %s", reqPath)
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.getLocalPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.getLocalPath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error) {
	fullPath, err := p.getLocalPath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		size := int64(0)
		if err == nil && !entry.IsDir() {
			size = info.Size()
		}

		results = append(results, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  size,
		})
	}

	return results, nil
}

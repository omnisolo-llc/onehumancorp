package mcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider provides local file system access bounded to a base directory.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{
		baseDir: baseDir,
	}
}

// resolvePath ensures the path is safely bounded to the base directory.
func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	absBase, err := filepath.Abs(p.baseDir)
	if err != nil {
		return "", fmt.Errorf("invalid base directory: %w", err)
	}

	absReq, err := filepath.Abs(filepath.Join(absBase, reqPath))
	if err != nil {
		return "", fmt.Errorf("invalid request path: %w", err)
	}

	rel, err := filepath.Rel(absBase, absReq)
	if err != nil {
		return "", fmt.Errorf("path is outside base directory")
	}

	// filepath.Rel returns ".." if the path is outside the base
	if rel == ".." || filepath.HasPrefix(filepath.ToSlash(rel), "../") {
		return "", fmt.Errorf("path traversal attempt detected")
	}

	return absReq, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // skip entries we can't stat
		}
		infos = append(infos, info)
	}
	return infos, nil
}

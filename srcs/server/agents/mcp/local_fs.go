package mcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("invalid base directory: %w", err)
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	cleanReq := strings.TrimPrefix(filepath.Clean(reqPath), "/")

	// Ensure that even if reqPath was "/etc/passwd", cleanReq won't be absolute anymore
	// because of TrimPrefix, but just to be sure we also check if filepath.IsAbs
	if filepath.IsAbs(reqPath) {
	    // If it's absolute, we want to reject it or make it relative to baseDir.
	    // Let's just make it relative by removing the leading slash and cleaning.
	    cleanReq = strings.TrimPrefix(filepath.Clean(reqPath), "/")
	}

	fullPath := filepath.Clean(filepath.Join(p.baseDir, cleanReq))

	// Safety check
	baseWithSep := p.baseDir + string(filepath.Separator)
	if !strings.HasPrefix(fullPath, baseWithSep) && fullPath != p.baseDir {
		return "", errors.New("path escapes base directory")
	}

	// Also reject if the requested path was absolute. Let's strictly enforce relative paths or paths that don't try to escape.
	// Actually, wait. If reqPath is "/etc/passwd", cleanReq is "etc/passwd".
	// filepath.Join(baseDir, "etc/passwd") = "/tmp/.../etc/passwd".
	// strings.HasPrefix("/tmp/.../etc/passwd", "/tmp/.../") is TRUE!
	// This means an absolute path bypasses our error return and just looks inside the local dir.
	// But our test expects an ERROR!
	if filepath.IsAbs(reqPath) {
	    return "", errors.New("path escapes base directory")
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

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(fullPath, func(currPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			if strings.Contains(d.Name(), pattern) {
				relPath, _ := filepath.Rel(p.baseDir, currPath)
				matches = append(matches, relPath)
			}
		}
		return nil
	})

	return matches, err
}

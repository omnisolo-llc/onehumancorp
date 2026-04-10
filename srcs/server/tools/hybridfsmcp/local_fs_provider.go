package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	absPath, _ := filepath.Abs(workspaceDir)
	return &LocalFSProvider{
		workspaceDir: absPath,
	}
}

// resolvePath ensures the path is bounded within the workspace directory.
func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	if p.workspaceDir == "" {
		return "", errors.New("workspace directory not configured")
	}

	joined := filepath.Join(p.workspaceDir, path)
	absPath, err := filepath.Abs(joined)
	if err != nil {
		return "", err
	}

	cleanWorkspace := filepath.Clean(p.workspaceDir)
	cleanTarget := filepath.Clean(absPath)

	if !strings.HasPrefix(cleanTarget, cleanWorkspace+string(filepath.Separator)) && cleanTarget != cleanWorkspace {
		return "", errors.New("path traversal detected: path escapes workspace directory")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolvedPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}

	var results []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // Skip entries we can't get info for
		}
		results = append(results, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}
	return results, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var results []string
	err = filepath.WalkDir(resolvedPath, func(walkPath string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil // Skip errors during walk
		}
		if d.IsDir() {
			return nil
		}

		match, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return nil
		}

		if match || strings.Contains(d.Name(), pattern) {
			// Return relative path
			relPath, err := filepath.Rel(p.workspaceDir, walkPath)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})

	return results, err
}

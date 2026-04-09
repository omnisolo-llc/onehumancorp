package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for Standalone mode, ensuring operations stay within a WorkspaceDir.
type LocalFSProvider struct {
	WorkspaceDir string
}

func NewLocalFSProvider(workspace string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspace)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{WorkspaceDir: absWorkspace}, nil
}

func (l *LocalFSProvider) sanitizePath(path string) (string, error) {
	cleanedPath := filepath.Clean(path)
	if filepath.IsAbs(cleanedPath) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	fullPath := filepath.Join(l.WorkspaceDir, cleanedPath)

	// Double check to prevent directory traversal
	if !strings.HasPrefix(fullPath, l.WorkspaceDir+string(filepath.Separator)) && fullPath != l.WorkspaceDir {
		return "", fmt.Errorf("path access denied")
	}

	return fullPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := l.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := l.sanitizePath(path)
	if err != nil {
		return err
	}

	// Ensure the parent directory exists
	if err := os.MkdirAll(filepath.Dir(safePath), 0700); err != nil {
		return err
	}

	// Create files with 0600 permissions
	return os.WriteFile(safePath, data, 0600)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := l.sanitizePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (l *LocalFSProvider) SearchFiles(ctx context.Context, directory string, pattern string) ([]string, error) {
	safeDir, err := l.sanitizePath(directory)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.Walk(safeDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			if matched, _ := filepath.Match(pattern, info.Name()); matched {
				relPath, _ := filepath.Rel(l.WorkspaceDir, path)
				matches = append(matches, relPath)
			}
		}
		return nil
	})

	return matches, err
}

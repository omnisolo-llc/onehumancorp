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

func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	// Ensure base path exists
	err = os.MkdirAll(absPath, 0755)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{basePath: absPath}, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) sanitizePath(inputPath string) (string, error) {
	cleanPath := filepath.Clean(inputPath)
	fullPath := filepath.Join(p.basePath, cleanPath)

	// Ensure the resulting path is still within basePath
	// Fix security vulnerability by comparing with Rel
	rel, err := filepath.Rel(p.basePath, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("invalid path: escapes base directory")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, query string, path string) ([]string, error) {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	_, statErr := os.Stat(fullPath)
	if statErr != nil {
		return nil, statErr
	}

	var results []string

	err = filepath.Walk(fullPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil // Skip errors in walk
		}

		if !info.IsDir() && strings.Contains(info.Name(), query) {
			relPath, err := filepath.Rel(p.basePath, path)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})

	return results, err
}

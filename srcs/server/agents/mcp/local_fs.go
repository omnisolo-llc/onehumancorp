package mcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider, restricting file access
// to paths strictly within the provided baseDir.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for baseDir: %w", err)
	}

	return &LocalFSProvider{
		baseDir: absBase,
	}, nil
}

// resolvePath resolves the given targetPath relative to the baseDir and verifies
// that it does not escape the base directory (path traversal check).
func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	// The problem memory says:
	// When implementing wrapper/delegate patterns for file system providers, avoid double-prefixing the base directory.
	// Pass only the relative scoped path to the delegate if the underlying provider inherently resolves absolute paths against the base directory.

	// Construct the absolute path
	fullPath := filepath.Join(p.baseDir, targetPath)

	// Clean it to resolve any ../
	cleanPath := filepath.Clean(fullPath)

	// Path traversal bounds check
	// Appending a separator prevents partial directory matching
	baseDirWithSep := p.baseDir + string(filepath.Separator)
	if !strings.HasPrefix(cleanPath, baseDirWithSep) && cleanPath != p.baseDir {
		return "", errors.New("path traversal violation: target path escapes base directory")
	}

	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	return os.ReadFile(resolvedPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolvedPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolvedPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}

	return names, nil
}

package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalFSProvider implements FileSystemProvider for the local file system.
// It is bounded to a specific root directory.
type LocalFSProvider struct {
	rootDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the given root directory.
func NewLocalFSProvider(rootDir string) (*LocalFSProvider, error) {
	absRoot, err := filepath.Abs(rootDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for root dir: %w", err)
	}
	// Ensure the directory exists
	if err := os.MkdirAll(absRoot, 0700); err != nil {
		return nil, fmt.Errorf("failed to create root directory: %w", err)
	}
	return &LocalFSProvider{rootDir: absRoot}, nil
}

// resolvePath resolves the given path against the root directory and ensures it does not escape.
func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	// If the path is absolute, joining it with rootDir will actually ignore rootDir.
	// So we clean it and strip leading separators to ensure it's treated as relative to rootDir.
	cleanInput := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanInput) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", targetPath)
	}

	absTarget := filepath.Join(p.rootDir, cleanInput)
	cleanTarget := filepath.Clean(absTarget)

	// Check if the resolved path is within the root directory
	rel, err := filepath.Rel(p.rootDir, cleanTarget)
	if err != nil {
		return "", fmt.Errorf("failed to determine relative path: %w", err)
	}
	if rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes root directory: %s", targetPath)
	}

	return cleanTarget, nil
}

// ReadFile reads the file at the given path.
func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolvedPath)
}

// WriteFile writes the given content to the file at the given path.
func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	resolvedPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	// Ensure the directory exists
	dir := filepath.Dir(resolvedPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return fmt.Errorf("failed to create parent directories: %w", err)
	}

	return os.WriteFile(resolvedPath, content, 0600)
}

// ListDir lists the files in the directory at the given path.
func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
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

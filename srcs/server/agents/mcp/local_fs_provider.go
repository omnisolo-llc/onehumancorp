package mcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider with strict path bounding
// to a given workspace directory to prevent escaping the local working directory.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, fmt.Errorf("invalid workspace directory: %w", err)
	}
	return &LocalFSProvider{workspaceDir: absWorkspace}, nil
}

// resolvePath securely resolves a path, ensuring it stays within the workspaceDir
func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.workspaceDir, target))
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}

	// Ensure prefix matching ends with separator to prevent prefix spoofing (e.g., /mnt/tenant vs /mnt/tenant-10)
	workspaceDirWithSep := p.workspaceDir
	if !strings.HasSuffix(workspaceDirWithSep, string(filepath.Separator)) {
		workspaceDirWithSep += string(filepath.Separator)
	}
	absPathWithSep := absPath
	if !strings.HasSuffix(absPathWithSep, string(filepath.Separator)) {
		absPathWithSep += string(filepath.Separator)
	}

	// It's safe if it perfectly matches the workspace dir, or has it as prefix
	if absPath != p.workspaceDir && !strings.HasPrefix(absPathWithSep, workspaceDirWithSep) {
		return "", fmt.Errorf("access denied: path escapes workspace bounds")
	}

	return absPath, nil
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

	// Ensure parent directory exists
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, fmt.Errorf("failed to read directory: %w", err)
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

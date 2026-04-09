package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for a local directory.
type LocalFSProvider struct {
	workspaceRoot string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to a specific directory.
func NewLocalFSProvider(workspaceRoot string) (*LocalFSProvider, error) {
	absRoot, err := filepath.Abs(workspaceRoot)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceRoot: absRoot}, nil
}

// securePath resolves a path and ensures it does not escape the workspace root.
func (p *LocalFSProvider) securePath(target string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.workspaceRoot, target))
	if err != nil {
		return "", err
	}

	// Clean the workspaceRoot to ensure it doesn't end with a separator, then add one
	// to strictly match the directory boundary, preventing sibling directory traversal.
	// For example, if workspaceRoot is /data/workspace, a request for /data/workspace_secrets
	// would pass a simple HasPrefix check, but fail if we check for HasPrefix(/data/workspace/).
	// We also need to allow the exact root path itself.
	rootWithSep := filepath.Clean(p.workspaceRoot) + string(filepath.Separator)

	if absPath != filepath.Clean(p.workspaceRoot) && !strings.HasPrefix(absPath, rootWithSep) {
		return "", errors.New("access denied: path escapes workspace bounds")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	secure, err := p.securePath(path)
	if err != nil {
		return err
	}
	// Ensure parent directory exists
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

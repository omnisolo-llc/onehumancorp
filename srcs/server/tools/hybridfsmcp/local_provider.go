package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
	return &LocalFSProvider{
		basePath: filepath.Clean(basePath),
	}
}

func (p *LocalFSProvider) securePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.basePath, path))

	// Ensure cleanPath is either exactly basePath or a child of basePath
	if cleanPath == p.basePath {
		return cleanPath, nil
	}

	basePathWithSeparator := p.basePath + string(filepath.Separator)
	if !strings.HasPrefix(cleanPath, basePathWithSeparator) {
		return "", fmt.Errorf("path escapes base directory: %s", path)
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	securePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(securePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	securePath, err := p.securePath(path)
	if err != nil {
		return err
	}
    if err := os.MkdirAll(filepath.Dir(securePath), 0755); err != nil {
        return err
    }
	return os.WriteFile(securePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	securePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(securePath)
	if err != nil {
		return nil, err
	}
	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

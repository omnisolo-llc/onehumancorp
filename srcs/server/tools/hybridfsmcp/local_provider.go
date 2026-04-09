package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
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
	return &LocalFSProvider{
		workspaceDir: filepath.Clean(workspaceDir),
	}
}

func (l *LocalFSProvider) resolveAndValidatePath(requestedPath string) (string, error) {
	absPath := filepath.Clean(filepath.Join(l.workspaceDir, requestedPath))

	// Ensure the path is within the workspace directory
	if !strings.HasPrefix(absPath, l.workspaceDir+string(filepath.Separator)) && absPath != l.workspaceDir {
		return "", errors.New("access denied: path traversal attempt")
	}

	return absPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	validPath, err := l.resolveAndValidatePath(path)
	if err != nil {
		return nil, err
	}

	return os.ReadFile(validPath)
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	validPath, err := l.resolveAndValidatePath(path)
	if err != nil {
		return err
	}

	// Ensure the directory exists
	if err := os.MkdirAll(filepath.Dir(validPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(validPath, data, 0644)
}

func (l *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	validPath, err := l.resolveAndValidatePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(validPath)
	if err != nil {
		return nil, err
	}

	var fileInfos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		fileInfos = append(fileInfos, info)
	}

	return fileInfos, nil
}

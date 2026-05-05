package storage

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type LocalBlobProvider struct {
	rootDir string
}

func NewLocalBlobProvider(rootDir string) (*LocalBlobProvider, error) {
	absRootDir, err := filepath.Abs(rootDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for root dir: %w", err)
	}

	// Ensure the directory exists
	if err := os.MkdirAll(absRootDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create root dir %s: %w", absRootDir, err)
	}

	return &LocalBlobProvider{rootDir: absRootDir}, nil
}

func (l *LocalBlobProvider) resolvePath(p string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(l.rootDir, p))
	if err != nil {
		return "", err
	}

	rootDirWithSeparator := l.rootDir + string(filepath.Separator)
	if !strings.HasPrefix(absPath+string(filepath.Separator), rootDirWithSeparator) {
		return "", fmt.Errorf("path escapes sandbox: %s", p)
	}

	return absPath, nil
}

func (l *LocalBlobProvider) WriteBlob(ctx context.Context, p string, data []byte) error {
	absPath, err := l.resolvePath(p)
	if err != nil {
		return err
	}

	// Ensure the directory exists
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory for %s: %w", absPath, err)
	}

	if err := os.WriteFile(absPath, data, 0644); err != nil {
		return fmt.Errorf("failed to write blob to %s: %w", absPath, err)
	}
	return nil
}

func (l *LocalBlobProvider) ReadBlob(ctx context.Context, p string) ([]byte, error) {
	absPath, err := l.resolvePath(p)
	if err != nil {
		return nil, err
	}

	data, err := os.ReadFile(absPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read blob from %s: %w", absPath, err)
	}
	return data, nil
}

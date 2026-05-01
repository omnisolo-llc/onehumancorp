package storage

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// LocalBlobProvider implements BlobProvider using the local filesystem.
type LocalBlobProvider struct {
	baseDir string
}

// NewLocalBlobProvider creates a new LocalBlobProvider.
func NewLocalBlobProvider(baseDir string) (*LocalBlobProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path: %w", err)
	}

	if err := os.MkdirAll(absBaseDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create base directory: %w", err)
	}

	return &LocalBlobProvider{baseDir: absBaseDir}, nil
}

func (p *LocalBlobProvider) securePath(path string) (string, error) {
	fullPath := filepath.Join(p.baseDir, path)
	cleanPath := filepath.Clean(fullPath)

	rel, err := filepath.Rel(p.baseDir, cleanPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path %s is outside base directory", path)
	}

	return cleanPath, nil
}

// WriteBlob writes data to a local file.
func (p *LocalBlobProvider) WriteBlob(ctx context.Context, path string, data []byte) error {
	securePath, err := p.securePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(securePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	if err := os.WriteFile(securePath, data, 0644); err != nil {
		return fmt.Errorf("failed to write file: %w", err)
	}

	return nil
}

// ReadBlob reads data from a local file.
func (p *LocalBlobProvider) ReadBlob(ctx context.Context, path string) ([]byte, error) {
	securePath, err := p.securePath(path)
	if err != nil {
		return nil, err
	}

	data, err := os.ReadFile(securePath)
	if err != nil {
		return nil, fmt.Errorf("failed to read file: %w", err)
	}

	return data, nil
}

package agents

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// OHCMemory defines the interface for local memory fallback.
type OHCMemory interface {
	Write(ctx context.Context, namespace, key string, data []byte) error
	Read(ctx context.Context, namespace, key string) ([]byte, error)
}

// FileBasedMemory implements OHCMemory using the local filesystem.
type FileBasedMemory struct {
	BaseDir string
}

// NewFileBasedMemory creates a new FileBasedMemory instance.
func NewFileBasedMemory(baseDir string) *FileBasedMemory {
	return &FileBasedMemory{BaseDir: baseDir}
}

// secureJoin safely joins the base directory with the provided paths to prevent path traversal.
func (f *FileBasedMemory) secureJoin(elem ...string) (string, error) {
	joined := filepath.Join(append([]string{f.BaseDir}, elem...)...)
	absBase, err := filepath.Abs(f.BaseDir)
	if err != nil {
		return "", err
	}
	absJoined, err := filepath.Abs(joined)
	if err != nil {
		return "", err
	}
	if !strings.HasPrefix(absJoined, absBase) {
		return "", fmt.Errorf("invalid path: attempts to traverse outside base directory")
	}
	return joined, nil
}

// Write writes data to the memory directory.
func (f *FileBasedMemory) Write(ctx context.Context, namespace, key string, data []byte) error {
	dir, err := f.secureJoin(namespace)
	if err != nil {
		return fmt.Errorf("path traversal attempt in namespace: %w", err)
	}
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}
	path, err := f.secureJoin(namespace, key)
	if err != nil {
		return fmt.Errorf("path traversal attempt in key: %w", err)
	}
	if err := os.WriteFile(path, data, 0644); err != nil {
		return fmt.Errorf("failed to write memory file: %w", err)
	}
	return nil
}

// Read reads data from the memory directory.
func (f *FileBasedMemory) Read(ctx context.Context, namespace, key string) ([]byte, error) {
	path, err := f.secureJoin(namespace, key)
	if err != nil {
		return nil, fmt.Errorf("path traversal attempt: %w", err)
	}
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read memory file: %w", err)
	}
	return data, nil
}

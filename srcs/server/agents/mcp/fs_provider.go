package mcp

import (
	"context"
	"io/fs"
)

// FileSystemProvider defines the standard interface for file operations
// across both Standalone and Cloud-Native modes.
type FileSystemProvider interface {
	// ReadFile reads the contents of the file at the given path.
	ReadFile(ctx context.Context, path string) ([]byte, error)
	// WriteFile writes the given data to the file at the given path.
	WriteFile(ctx context.Context, path string, data []byte, perm fs.FileMode) error
	// ListDir lists the contents of the directory at the given path.
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

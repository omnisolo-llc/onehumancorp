package hybridfsmcp

import (
	"context"
	"io/fs"
)

// FileSystemProvider defines the interface for unified file system access.
type FileSystemProvider interface {
	// IsLocal returns true if the provider is a local filesystem.
	IsLocal() bool
	// ReadFile reads the contents of a file.
	ReadFile(ctx context.Context, path string) ([]byte, error)
	// WriteFile writes data to a file.
	WriteFile(ctx context.Context, path string, data []byte) error
	// ListDir lists the contents of a directory.
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
	// SearchFiles searches for files matching a pattern.
	SearchFiles(ctx context.Context, directory string, pattern string) ([]string, error)
}

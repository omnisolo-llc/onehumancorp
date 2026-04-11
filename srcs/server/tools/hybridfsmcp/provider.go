package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the interface for unified file system access.
type FileSystemProvider interface {
	// IsLocal returns true if the provider maps to a local Standalone filesystem.
	IsLocal() bool

	// ReadFile reads the contents of the given file.
	ReadFile(ctx context.Context, path string) ([]byte, error)

	// WriteFile writes data to the given file, creating it if it doesn't exist.
	WriteFile(ctx context.Context, path string, data []byte) error

	// ListDir returns a list of files and directories within the given path.
	ListDir(ctx context.Context, path string) ([]string, error)

	// SearchFiles recursively searches for files matching a pattern starting from the given path.
	SearchFiles(ctx context.Context, path string, pattern string) ([]string, error)
}

package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the interface for hybrid file system operations
// which abstracts local and cloud implementations.
type FileSystemProvider interface {
	// ReadFile reads the contents of the file at the given path.
	ReadFile(ctx context.Context, path string) ([]byte, error)
	// WriteFile writes the given data to the file at the given path.
	WriteFile(ctx context.Context, path string, data []byte) error
	// ListDir lists the contents of the directory at the given path.
	ListDir(ctx context.Context, path string) ([]string, error)
	// SearchFiles searches for files matching the query pattern in the given directory path.
	SearchFiles(ctx context.Context, path string, pattern string) ([]string, error)
}

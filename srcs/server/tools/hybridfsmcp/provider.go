package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the interface for hybrid file system operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

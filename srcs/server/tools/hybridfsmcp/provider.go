package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the interface for interacting with a file system
// in both local and cloud modes.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

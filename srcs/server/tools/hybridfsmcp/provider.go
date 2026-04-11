package hybridfsmcp

import (
	"context"
)

// FileInfo represents metadata for a file or directory.
type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

// FileSystemProvider defines the interface for hybrid file system access.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

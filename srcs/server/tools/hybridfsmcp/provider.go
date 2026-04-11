package hybridfsmcp

import (
	"context"
)

// FileInfo provides metadata about a file or directory.
type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

// FileSystemProvider abstracts file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
	IsLocal() bool
}

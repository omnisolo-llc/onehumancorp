package hybridfsmcp

import (
	"context"
)

// FileInfo provides standard information about a file
type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

// FileSystemProvider defines the unified interface for file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

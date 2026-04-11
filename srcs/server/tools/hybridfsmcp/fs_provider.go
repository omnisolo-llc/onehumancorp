package hybridfsmcp

import (
	"context"
	"io/fs"
)

// FileSystemProvider defines the interface for hybrid file system access.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

package hybridfsmcp

import (
	"context"
	"io/fs"
)

// FileSystemProvider defines the interface for File System operations
// across different environments (Cloud vs Local).
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) (string, error)
	WriteFile(ctx context.Context, path string, content string) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
}

package hybridfsmcp

import (
	"context"
	"io/fs"
)

// FileSystemProvider abstracts file system operations to support hybrid environments.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte, perm fs.FileMode) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

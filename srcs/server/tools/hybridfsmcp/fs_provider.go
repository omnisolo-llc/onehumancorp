package hybridfsmcp

import (
	"context"
)

type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

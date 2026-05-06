package hybridfsmcp

import (
	"context"
)

type FileSystemProvider interface {
	IsLocal() bool
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, query string, path string) ([]string, error)
}

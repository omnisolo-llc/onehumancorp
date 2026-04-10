package hybridfsmcp

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileInfo represents metadata about a file or directory.
type FileInfo struct {
	Name  string `json:"name"`
	IsDir bool   `json:"is_dir"`
	Size  int64  `json:"size"`
}

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error)
}

package hybridfsmcp

import (
	"context"
)

// FileInfo represents the metadata of a file or directory.
type FileInfo struct {
	Name    string `json:"name"`
	IsDir   bool   `json:"is_dir"`
	Size    int64  `json:"size"`
	ModTime string `json:"mod_time"`
}

// FileSystemProvider defines the interface for hybrid file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
	SearchFiles(ctx context.Context, pattern string) ([]string, error)
}

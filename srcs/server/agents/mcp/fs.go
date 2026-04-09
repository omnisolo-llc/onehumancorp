package mcp

import (
	"context"
)

// FileInfo represents metadata about a file or directory.
type FileInfo struct {
	Name  string `json:"name"`
	IsDir bool   `json:"is_dir"`
	Size  int64  `json:"size"`
}

// FileSystemProvider abstracts file operations for local and cloud modes.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
}

package mcp

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

// FileSystemProvider defines the interface for interacting with file systems.
type FileSystemProvider interface {
	// ReadFile reads the contents of a file at the given path.
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	// WriteFile writes data to a file at the given path, creating it if necessary.
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	// ListDir lists the contents of a directory.
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error)
}

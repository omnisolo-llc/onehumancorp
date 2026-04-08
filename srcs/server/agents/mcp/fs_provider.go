package mcp

import (
	"context"
	"io/fs"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts the underlying file system operations
// for both Local (Standalone) and Cloud (Multi-tenant) modes.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error)
}

package mcp

import (
	"context"
)

// FileSystemProvider abstracts file system operations to support
// both local file access (Standalone Mode) and cloud/tenant-scoped access (Cloud Mode).
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

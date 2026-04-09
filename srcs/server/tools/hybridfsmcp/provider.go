package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines an interface for an MCP server to access a virtualized filesystem.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, directory string, pattern string) ([]string, error)
}

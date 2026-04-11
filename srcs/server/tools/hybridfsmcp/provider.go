package hybridfsmcp

import "context"

// FileSystemProvider abstracts file system operations for the MCP server.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

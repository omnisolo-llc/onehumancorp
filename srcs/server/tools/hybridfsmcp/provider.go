package hybridfsmcp

import (
	"context"
	"io/fs"
)

// FileSystemProvider abstracts file operations for the Swarm MCP.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
}

package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the standard interface for file operations
// exposed to MCP agents across cloud and standalone modes.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

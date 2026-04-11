package mcp

import "context"

// FileSystemProvider defines an interface for an abstraction of a file system
// that agents interact with, providing safe, scoped file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

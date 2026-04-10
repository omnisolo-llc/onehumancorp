package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the interface for File System operations
// in both local (Standalone) and cloud (Multi-Tenant) environments.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

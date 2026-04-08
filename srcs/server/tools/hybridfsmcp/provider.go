package hybridfsmcp

import (
	"context"
)

// FileSystemProvider defines the interface for interacting with the hybrid file system
type FileSystemProvider interface {
	ReadFile(ctx context.Context, tenantID, path string) ([]byte, error)
	WriteFile(ctx context.Context, tenantID, path string, data []byte) error
	ListDir(ctx context.Context, tenantID, path string) ([]string, error)
	SearchFiles(ctx context.Context, tenantID, path, pattern string) ([]string, error)
}

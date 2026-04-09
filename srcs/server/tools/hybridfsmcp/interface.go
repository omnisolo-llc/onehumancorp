package hybridfsmcp

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

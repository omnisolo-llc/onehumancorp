package hybridfsmcp

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts the file system access for both local and cloud environments.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error)
}

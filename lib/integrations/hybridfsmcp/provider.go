package hybridfsmcp

import (
    "context"
    "github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for unified file system access.
type FileSystemProvider interface {
    // IsLocal returns true if the provider is a local filesystem.
    IsLocal() bool
    // ReadFile reads the contents of the given file path.
    ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
    // WriteFile writes the given contents to the given file path.
    WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error
    // ListDir lists the files in the given directory path.
    ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

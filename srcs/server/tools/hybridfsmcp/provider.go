package hybridfsmcp

import (
    "context"
)

type FileSystemProvider interface {
    ReadFile(ctx context.Context, path string) ([]byte, error)
    WriteFile(ctx context.Context, path string, data []byte) error
    ListDir(ctx context.Context, path string) ([]map[string]interface{}, error)
    IsLocal() bool
}

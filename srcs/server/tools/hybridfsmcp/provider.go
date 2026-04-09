package hybridfsmcp

import "context"

type FileSystemProvider interface {
    ReadFile(ctx context.Context, key string) ([]byte, error)
    WriteFile(ctx context.Context, key string, content []byte) error
    ListDir(ctx context.Context, prefix string) ([]string, error)
    SearchFiles(ctx context.Context, pattern string) ([]string, error)
}

package hybridfsmcp

import (
	"context"
	"os"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

func NewProvider(ctx context.Context, baseDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return &LocalFSProvider{BaseDir: baseDir}
	}
	return &CloudFSProvider{BaseDir: baseDir}
}

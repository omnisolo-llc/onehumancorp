package mcp

import (
	"context"
)

// BlobProvider defines the abstraction for writing and reading blobs
// regardless of the underlying infrastructure (S3 or Local FS).
type BlobProvider interface {
	WriteBlob(ctx context.Context, key string, data []byte) error
	ReadBlob(ctx context.Context, key string) ([]byte, error)
}

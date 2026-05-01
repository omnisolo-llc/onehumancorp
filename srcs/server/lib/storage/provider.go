package storage

import "context"

// BlobProvider defines the interface for blob storage operations.
type BlobProvider interface {
	WriteBlob(ctx context.Context, path string, data []byte) error
	ReadBlob(ctx context.Context, path string) ([]byte, error)
}

package storage

import "context"

type BlobProvider interface {
	WriteBlob(ctx context.Context, path string, data []byte) error
	ReadBlob(ctx context.Context, path string) ([]byte, error)
}

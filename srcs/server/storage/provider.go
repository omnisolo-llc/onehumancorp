package storage

import (
	"context"
	"time"
)

// BlobMetadata represents metadata for a stored blob.
type BlobMetadata struct {
	Key          string
	Size         int64
	LastModified time.Time
	ContentType  string
}

// Provider defines the interface for unified blob storage access.
type Provider interface {
	// IsLocal returns true if the provider is a local filesystem.
	IsLocal() bool
	// ListBlobs returns a list of blob metadata under a given prefix.
	ListBlobs(ctx context.Context, prefix string) ([]BlobMetadata, error)
	// ReadBlobMetadata returns the metadata for a single blob.
	ReadBlobMetadata(ctx context.Context, key string) (BlobMetadata, error)
	// GetBlobURL returns a presigned or accessible URL for the blob.
	GetBlobURL(ctx context.Context, key string) (string, error)
}

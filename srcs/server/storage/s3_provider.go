package storage

import (
	"context"
	"fmt"
	"time"
)

// S3Provider implements Provider for S3-compatible cloud storage.
// This is a stub implementation for the purpose of the MCP.
type S3Provider struct {
	bucketName string
	// S3 client would go here (e.g. *s3.Client or *minio.Client)
}

func NewS3Provider(bucketName string) *S3Provider {
	return &S3Provider{bucketName: bucketName}
}

func (p *S3Provider) IsLocal() bool {
	return false
}

func (p *S3Provider) ListBlobs(ctx context.Context, prefix string) ([]BlobMetadata, error) {
	// STUB
	return []BlobMetadata{}, nil
}

func (p *S3Provider) ReadBlobMetadata(ctx context.Context, key string) (BlobMetadata, error) {
	// STUB
	return BlobMetadata{
		Key:          key,
		Size:         1024,
		LastModified: time.Now(),
		ContentType:  "application/octet-stream",
	}, nil
}

func (p *S3Provider) GetBlobURL(ctx context.Context, key string) (string, error) {
	// STUB: Return a fake presigned URL
	return fmt.Sprintf("https://s3.amazonaws.com/%s/%s?X-Amz-Signature=stub", p.bucketName, key), nil
}

package storage

import (
	"bytes"
	"context"
	"fmt"
	"io"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/service/s3"
)

// S3API defines the S3 operations used by S3BlobProvider to allow for mocking in tests.
type S3API interface {
	PutObject(ctx context.Context, params *s3.PutObjectInput, optFns ...func(*s3.Options)) (*s3.PutObjectOutput, error)
	GetObject(ctx context.Context, params *s3.GetObjectInput, optFns ...func(*s3.Options)) (*s3.GetObjectOutput, error)
}

// S3BlobProvider implements BlobProvider using AWS S3.
type S3BlobProvider struct {
	client S3API
	bucket string
}

// NewS3BlobProvider creates a new S3BlobProvider.
func NewS3BlobProvider(client S3API, bucket string) *S3BlobProvider {
	return &S3BlobProvider{
		client: client,
		bucket: bucket,
	}
}

// WriteBlob writes data to an S3 object.
func (p *S3BlobProvider) WriteBlob(ctx context.Context, path string, data []byte) error {
	_, err := p.client.PutObject(ctx, &s3.PutObjectInput{
		Bucket: aws.String(p.bucket),
		Key:    aws.String(path),
		Body:   bytes.NewReader(data),
	})
	if err != nil {
		return fmt.Errorf("failed to write object to S3: %w", err)
	}

	return nil
}

// ReadBlob reads data from an S3 object.
func (p *S3BlobProvider) ReadBlob(ctx context.Context, path string) ([]byte, error) {
	out, err := p.client.GetObject(ctx, &s3.GetObjectInput{
		Bucket: aws.String(p.bucket),
		Key:    aws.String(path),
	})
	if err != nil {
		return nil, fmt.Errorf("failed to read object from S3: %w", err)
	}
	defer out.Body.Close()

	data, err := io.ReadAll(out.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read object body: %w", err)
	}

	return data, nil
}

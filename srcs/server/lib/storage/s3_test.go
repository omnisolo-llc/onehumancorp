package storage_test

import (
	"bytes"
	"context"
	"errors"
	"io"
	"testing"

	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"onehumancorp/srcs/server/lib/storage"
)

type mockS3Client struct {
	putErr error
	getErr error
	getData []byte
	readErr error
}

// errorReader returns an error when Read is called
type errorReader struct{}

func (e *errorReader) Read(p []byte) (n int, err error) {
	return 0, errors.New("read error")
}

func (m *mockS3Client) PutObject(ctx context.Context, params *s3.PutObjectInput, optFns ...func(*s3.Options)) (*s3.PutObjectOutput, error) {
	if m.putErr != nil {
		return nil, m.putErr
	}
	return &s3.PutObjectOutput{}, nil
}

func (m *mockS3Client) GetObject(ctx context.Context, params *s3.GetObjectInput, optFns ...func(*s3.Options)) (*s3.GetObjectOutput, error) {
	if m.getErr != nil {
		return nil, m.getErr
	}

	var body io.ReadCloser
	if m.readErr != nil {
		body = io.NopCloser(&errorReader{})
	} else {
		body = io.NopCloser(bytes.NewReader(m.getData))
	}

	return &s3.GetObjectOutput{
		Body: body,
	}, nil
}

func TestS3BlobProvider(t *testing.T) {
	mockClient := &mockS3Client{
		getData: []byte("s3 data"),
	}
	bucket := "test-bucket"
	provider := storage.NewS3BlobProvider(mockClient, bucket)

	ctx := context.Background()

	err := provider.WriteBlob(ctx, "test/path", []byte("s3 data"))
	require.NoError(t, err)

	data, err := provider.ReadBlob(ctx, "test/path")
	require.NoError(t, err)
	assert.Equal(t, []byte("s3 data"), data)
}

func TestS3BlobProvider_Errors(t *testing.T) {
	mockClient := &mockS3Client{
		putErr: errors.New("put error"),
		getErr: errors.New("get error"),
	}
	bucket := "test-bucket"
	provider := storage.NewS3BlobProvider(mockClient, bucket)

	ctx := context.Background()

	err := provider.WriteBlob(ctx, "test/path", []byte("s3 data"))
	require.Error(t, err)
	assert.Contains(t, err.Error(), "put error")

	_, err = provider.ReadBlob(ctx, "test/path")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "get error")

	// Test read error
	mockClient2 := &mockS3Client{
		readErr: errors.New("read error"),
	}
	provider2 := storage.NewS3BlobProvider(mockClient2, bucket)
	_, err = provider2.ReadBlob(ctx, "test/path")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "read object body")
}

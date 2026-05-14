package storage_test

import (
	"bytes"
	"context"
	"io"
	"testing"

	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"onehumancorp/srcs/server/lib/storage"
)

type MockS3Client struct {
	mock.Mock
}

func (m *MockS3Client) PutObject(ctx context.Context, params *s3.PutObjectInput, optFns ...func(*s3.Options)) (*s3.PutObjectOutput, error) {
	args := m.Called(ctx, params)
	if args.Get(0) != nil {
		return args.Get(0).(*s3.PutObjectOutput), args.Error(1)
	}
	return nil, args.Error(1)
}

func (m *MockS3Client) GetObject(ctx context.Context, params *s3.GetObjectInput, optFns ...func(*s3.Options)) (*s3.GetObjectOutput, error) {
	args := m.Called(ctx, params)
	if args.Get(0) != nil {
		return args.Get(0).(*s3.GetObjectOutput), args.Error(1)
	}
	return nil, args.Error(1)
}

func TestS3BlobProvider(t *testing.T) {
	mockClient := new(MockS3Client)
	bucket := "test-bucket"
	provider := storage.NewS3BlobProvider(mockClient, bucket)

	ctx := context.Background()
	path := "test-key.txt"
	data := []byte("hello s3")

	// Test Write
	mockClient.On("PutObject", ctx, mock.MatchedBy(func(params *s3.PutObjectInput) bool {
		buf := new(bytes.Buffer)
		buf.ReadFrom(params.Body)
		return *params.Bucket == bucket && *params.Key == path && bytes.Equal(buf.Bytes(), data)
	})).Return(&s3.PutObjectOutput{}, nil)

	err := provider.WriteBlob(ctx, path, data)
	assert.NoError(t, err)

	// Test Read
	mockClient.On("GetObject", ctx, mock.MatchedBy(func(params *s3.GetObjectInput) bool {
		return *params.Bucket == bucket && *params.Key == path
	})).Return(&s3.GetObjectOutput{
		Body: io.NopCloser(bytes.NewReader(data)),
	}, nil)

	readData, err := provider.ReadBlob(ctx, path)
	assert.NoError(t, err)
	assert.Equal(t, data, readData)

	mockClient.AssertExpectations(t)
}

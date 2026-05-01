package storage

import (
	"bytes"
	"context"
	"errors"
	"io"
	"testing"

	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

// MockS3Client is a mock of the S3API interface
type MockS3Client struct {
	mock.Mock
}

func (m *MockS3Client) PutObject(ctx context.Context, params *s3.PutObjectInput, optFns ...func(*s3.Options)) (*s3.PutObjectOutput, error) {
	args := m.Called(ctx, params)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*s3.PutObjectOutput), args.Error(1)
}

func (m *MockS3Client) GetObject(ctx context.Context, params *s3.GetObjectInput, optFns ...func(*s3.Options)) (*s3.GetObjectOutput, error) {
	args := m.Called(ctx, params)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(*s3.GetObjectOutput), args.Error(1)
}

func TestS3BlobProvider_WriteBlob(t *testing.T) {
	mockClient := new(MockS3Client)
	provider := NewS3BlobProvider(mockClient, "test-bucket")

	ctx := context.Background()
	testPath := "test/file.txt"
	testData := []byte("hello s3")

	// Setup mock expectation
	mockClient.On("PutObject", ctx, mock.AnythingOfType("*s3.PutObjectInput")).Return(&s3.PutObjectOutput{}, nil).Run(func(args mock.Arguments) {
		input := args.Get(1).(*s3.PutObjectInput)
		assert.Equal(t, "test-bucket", *input.Bucket)
		assert.Equal(t, testPath, *input.Key)

		bodyBytes, _ := io.ReadAll(input.Body)
		assert.Equal(t, testData, bodyBytes)
	})

	err := provider.WriteBlob(ctx, testPath, testData)
	assert.NoError(t, err)

	mockClient.AssertExpectations(t)
}

func TestS3BlobProvider_WriteBlob_Error(t *testing.T) {
	mockClient := new(MockS3Client)
	provider := NewS3BlobProvider(mockClient, "test-bucket")

	ctx := context.Background()

	expectedErr := errors.New("s3 error")
	mockClient.On("PutObject", ctx, mock.AnythingOfType("*s3.PutObjectInput")).Return(nil, expectedErr)

	err := provider.WriteBlob(ctx, "test/path", []byte("data"))
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "failed to write object to S3")
	assert.Contains(t, err.Error(), expectedErr.Error())
}

func TestS3BlobProvider_ReadBlob(t *testing.T) {
	mockClient := new(MockS3Client)
	provider := NewS3BlobProvider(mockClient, "test-bucket")

	ctx := context.Background()
	testPath := "test/file.txt"
	testData := []byte("hello s3")

	// Setup mock expectation
	mockClient.On("GetObject", ctx, mock.AnythingOfType("*s3.GetObjectInput")).Return(&s3.GetObjectOutput{
		Body: io.NopCloser(bytes.NewReader(testData)),
	}, nil).Run(func(args mock.Arguments) {
		input := args.Get(1).(*s3.GetObjectInput)
		assert.Equal(t, "test-bucket", *input.Bucket)
		assert.Equal(t, testPath, *input.Key)
	})

	data, err := provider.ReadBlob(ctx, testPath)
	assert.NoError(t, err)
	assert.Equal(t, testData, data)

	mockClient.AssertExpectations(t)
}

func TestS3BlobProvider_ReadBlob_Error(t *testing.T) {
	mockClient := new(MockS3Client)
	provider := NewS3BlobProvider(mockClient, "test-bucket")

	ctx := context.Background()

	expectedErr := errors.New("s3 error")
	mockClient.On("GetObject", ctx, mock.AnythingOfType("*s3.GetObjectInput")).Return(nil, expectedErr)

	_, err := provider.ReadBlob(ctx, "test/path")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "failed to read object from S3")
	assert.Contains(t, err.Error(), expectedErr.Error())
}

type errReader struct{}
func (errReader) Read(p []byte) (n int, err error) {
    return 0, errors.New("read error")
}
func (errReader) Close() error {
    return nil
}

func TestS3BlobProvider_ReadBlob_BodyReadError(t *testing.T) {
    mockClient := new(MockS3Client)
	provider := NewS3BlobProvider(mockClient, "test-bucket")

	ctx := context.Background()

    // Setup mock expectation returning an error reader
	mockClient.On("GetObject", ctx, mock.AnythingOfType("*s3.GetObjectInput")).Return(&s3.GetObjectOutput{
		Body: errReader{},
	}, nil)

    _, err := provider.ReadBlob(ctx, "test/path")
	assert.Error(t, err)
    assert.Contains(t, err.Error(), "failed to read object body")
}

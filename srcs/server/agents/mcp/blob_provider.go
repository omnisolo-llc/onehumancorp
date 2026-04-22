package mcp

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
)

// BlobProvider defines the interface for unified blob storage access in MCP.
type BlobProvider interface {
	WriteBlob(ctx context.Context, key string, data []byte) error
	ReadBlob(ctx context.Context, key string) ([]byte, error)
}

// LocalBlobProvider implements BlobProvider for the local filesystem.
type LocalBlobProvider struct {
	basePath string
}

func NewLocalBlobProvider(basePath string) (*LocalBlobProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(absPath, 0755); err != nil {
		return nil, err
	}
	return &LocalBlobProvider{basePath: absPath}, nil
}

func (p *LocalBlobProvider) getLocalPath(key string) (string, error) {
	cleanKey := filepath.Clean(key)
	if filepath.IsAbs(cleanKey) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	fullPath := filepath.Join(p.basePath, cleanKey)
	cleanPath := filepath.Clean(fullPath)

	if cleanPath != p.basePath && !strings.HasPrefix(cleanPath, p.basePath+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal detected")
	}

	return cleanPath, nil
}

func (p *LocalBlobProvider) WriteBlob(ctx context.Context, key string, data []byte) error {
	path, err := p.getLocalPath(key)
	if err != nil {
		return err
	}
	// Ensure parent dir exists
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return err
	}
	return os.WriteFile(path, data, 0644)
}

func (p *LocalBlobProvider) ReadBlob(ctx context.Context, key string) ([]byte, error) {
	path, err := p.getLocalPath(key)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(path)
}

// S3BlobProvider implements BlobProvider for S3.
type S3BlobProvider struct {
	bucketName string
	client     *minio.Client
}

func NewS3BlobProvider(bucketName string, endpoint string, accessKey string, secretKey string, secure bool) (*S3BlobProvider, error) {
	client, err := minio.New(endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(accessKey, secretKey, ""),
		Secure: secure,
	})
	if err != nil {
		return nil, err
	}
	return &S3BlobProvider{bucketName: bucketName, client: client}, nil
}

func (p *S3BlobProvider) WriteBlob(ctx context.Context, key string, data []byte) error {
	reader := bytes.NewReader(data)
	_, err := p.client.PutObject(ctx, p.bucketName, key, reader, int64(len(data)), minio.PutObjectOptions{
		ContentType: "application/octet-stream",
	})
	return err
}

func (p *S3BlobProvider) ReadBlob(ctx context.Context, key string) ([]byte, error) {
	object, err := p.client.GetObject(ctx, p.bucketName, key, minio.GetObjectOptions{})
	if err != nil {
		return nil, err
	}
	defer object.Close()

	data, err := io.ReadAll(object)
	if err != nil {
		errResp := minio.ToErrorResponse(err)
		if errResp.Code == "NoSuchKey" {
			return nil, os.ErrNotExist
		}
		return nil, err
	}
	return data, nil
}
// NewBlobProvider returns the correct BlobProvider based on environment variables.
func NewBlobProvider() (BlobProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalBlobProvider("/var/tmp/ohc/blobs")
	} else if os.Getenv("OHC_MULTITENANT") == "true" {
		endpoint := os.Getenv("S3_ENDPOINT")
		accessKey := os.Getenv("S3_ACCESS_KEY")
		secretKey := os.Getenv("S3_SECRET_KEY")
		secureStr := os.Getenv("S3_SECURE")
		secure := true
		if strings.ToLower(secureStr) == "false" || secureStr == "0" {
			secure = false
		}

		if endpoint == "" {
			return nil, fmt.Errorf("S3_ENDPOINT is required in Cloud Mode")
		}

		bucketName := "ohc-multi-tenant-blobs"

		provider, err := NewS3BlobProvider(bucketName, endpoint, accessKey, secretKey, secure)
		if err != nil {
			return nil, err
		}
		return provider, nil
	}
	// Default to local if nothing specified
	return NewLocalBlobProvider("/var/tmp/ohc/blobs")
}
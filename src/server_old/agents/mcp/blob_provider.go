package mcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
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
}

func NewS3BlobProvider(bucketName string) *S3BlobProvider {
	return &S3BlobProvider{bucketName: bucketName}
}

func (p *S3BlobProvider) WriteBlob(ctx context.Context, key string, data []byte) error {
	// STUB
	return nil
}

func (p *S3BlobProvider) ReadBlob(ctx context.Context, key string) ([]byte, error) {
	// STUB
	return []byte("stub data"), nil
}

// NewBlobProvider returns the correct BlobProvider based on environment variables.
func NewBlobProvider() (BlobProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalBlobProvider("/var/tmp/ohc/blobs")
	} else if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewS3BlobProvider("ohc-multi-tenant-blobs"), nil
	}
	// Default to local if nothing specified
	return NewLocalBlobProvider("/var/tmp/ohc/blobs")
}

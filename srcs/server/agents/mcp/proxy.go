package mcp

import (
	"context"
	"fmt"
	"os"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/s3"

	"onehumancorp/srcs/server/lib/storage"
)

// NewBlobProxy creates a new BlobProvider based on environment variables.
// It checks OHC_STANDALONE and OHC_MULTITENANT.
func NewBlobProxy(ctx context.Context) (storage.BlobProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		// Use local filesystem for standalone mode.
		baseDir := os.Getenv("OHC_LOCAL_STORAGE_DIR")
		if baseDir == "" {
			baseDir = "/tmp/ohc_storage" // Fallback default
		}
		return storage.NewLocalBlobProvider(baseDir)
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		// Use S3 for multitenant cloud mode.
		bucket := os.Getenv("AWS_S3_BUCKET")
		if bucket == "" {
			return nil, fmt.Errorf("AWS_S3_BUCKET environment variable must be set in multitenant mode")
		}

		cfg, err := config.LoadDefaultConfig(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to load AWS config: %w", err)
		}

		client := s3.NewFromConfig(cfg)
		return storage.NewS3BlobProvider(client, bucket), nil
	}

	// Default to local if neither is explicitly set but warn or fail?
	// The prompt implies routing based on the active mode.
	// If neither is set, we return an error.
	return nil, fmt.Errorf("neither OHC_STANDALONE nor OHC_MULTITENANT is set")
}

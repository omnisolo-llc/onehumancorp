package mcp

import (
	"context"
	"fmt"
	"os"
	"time"
	"onehumancorp/srcs/server/telemetry"

	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"onehumancorp/srcs/server/lib/storage"
)

type BlobProvider interface {
	WriteBlob(ctx context.Context, path string, data []byte) error
	ReadBlob(ctx context.Context, path string) ([]byte, error)
}

func NewBlobProxy(ctx context.Context) (BlobProvider, error) {
	start := time.Now()
	defer func() { telemetry.RecordHarnessInitLatency(ctx, time.Since(start).Seconds()) }()
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	isMultitenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultitenant && !isStandalone {
		bucket := os.Getenv("OHC_S3_BUCKET")
		if bucket == "" {
			bucket = "ohc-multi-tenant-blobs"
		}

		cfg, err := config.LoadDefaultConfig(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to load AWS config: %w", err)
		}

		client := s3.NewFromConfig(cfg)
		return storage.NewS3BlobProvider(client, bucket), nil
	}

	rootDir := os.Getenv("OHC_LOCAL_STORAGE_ROOT")
	if rootDir == "" {
		rootDir = "/var/tmp/ohc/blobs"
	}

	provider, err := storage.NewLocalBlobProvider(rootDir)
	if err != nil {
		return nil, fmt.Errorf("failed to create local blob provider: %w", err)
	}

	return provider, nil
}

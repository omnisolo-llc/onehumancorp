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

func NewBlobProxy(ctx context.Context) (storage.BlobProvider, error) {
	start := time.Now()
	defer func() { telemetry.RecordHarnessInitLatency(ctx, time.Since(start).Seconds()) }()
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	isMultitenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultitenant && !isStandalone {
		bucket := os.Getenv("OHC_S3_BUCKET")
		if bucket == "" {
			return nil, fmt.Errorf("OHC_S3_BUCKET must be set when OHC_MULTITENANT is true")
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
		rootDir = "./.local_storage"
	}

	provider, err := storage.NewLocalBlobProvider(rootDir)
	if err != nil {
		return nil, fmt.Errorf("failed to create local blob provider: %w", err)
	}

	return provider, nil
}

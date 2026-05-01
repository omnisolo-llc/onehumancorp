package mcp

import (
	"context"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"onehumancorp/srcs/server/lib/storage"
)

func TestNewBlobProxy_Standalone(t *testing.T) {
	// Setup env
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_MULTITENANT", "")

	tmpDir, err := os.MkdirTemp("", "proxy_test")
	require.NoError(t, err)
	defer os.RemoveAll(tmpDir)

	t.Setenv("OHC_LOCAL_STORAGE_DIR", tmpDir)

	ctx := context.Background()
	provider, err := NewBlobProxy(ctx)

	assert.NoError(t, err)
	assert.NotNil(t, provider)
	_, ok := provider.(*storage.LocalBlobProvider)
	assert.True(t, ok, "Expected LocalBlobProvider")
}

func TestNewBlobProxy_Standalone_DefaultDir(t *testing.T) {
	// Setup env
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_LOCAL_STORAGE_DIR", "")

	ctx := context.Background()
	provider, err := NewBlobProxy(ctx)

	assert.NoError(t, err)
	assert.NotNil(t, provider)
	_, ok := provider.(*storage.LocalBlobProvider)
	assert.True(t, ok, "Expected LocalBlobProvider")
}

func TestNewBlobProxy_Multitenant(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "")
	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("AWS_S3_BUCKET", "test-bucket")

	// Set dummy AWS credentials to avoid config load error
	t.Setenv("AWS_REGION", "us-east-1")
	t.Setenv("AWS_ACCESS_KEY_ID", "dummy")
	t.Setenv("AWS_SECRET_ACCESS_KEY", "dummy")

	ctx := context.Background()
	provider, err := NewBlobProxy(ctx)

	assert.NoError(t, err)
	assert.NotNil(t, provider)
	_, ok := provider.(*storage.S3BlobProvider)
	assert.True(t, ok, "Expected S3BlobProvider")
}

func TestNewBlobProxy_Multitenant_MissingBucket(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "")
	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("AWS_S3_BUCKET", "") // Missing

	ctx := context.Background()
	provider, err := NewBlobProxy(ctx)

	assert.Error(t, err)
	assert.Nil(t, provider)
	assert.Contains(t, err.Error(), "AWS_S3_BUCKET environment variable must be set")
}

func TestNewBlobProxy_NoModeSet(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "")
	t.Setenv("OHC_MULTITENANT", "")

	ctx := context.Background()
	provider, err := NewBlobProxy(ctx)

	assert.Error(t, err)
	assert.Nil(t, provider)
	assert.Contains(t, err.Error(), "neither OHC_STANDALONE nor OHC_MULTITENANT is set")
}

package mcp_test

import (
	"context"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"onehumancorp/srcs/server/agents/mcp"
	"onehumancorp/srcs/server/lib/storage"
)

func TestNewBlobProxy(t *testing.T) {
	ctx := context.Background()

	// Helper to restore env
	restoreEnv := func() {
		os.Unsetenv("OHC_STANDALONE")
		os.Unsetenv("OHC_MULTITENANT")
		os.Unsetenv("OHC_S3_BUCKET")
		os.Unsetenv("OHC_LOCAL_STORAGE_ROOT")
	}

	t.Run("Standalone Mode", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_STANDALONE", "true")

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.LocalBlobProvider{}, provider)
	})

	t.Run("Multitenant Mode with S3 Bucket", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_MULTITENANT", "true")
		os.Setenv("OHC_S3_BUCKET", "test-bucket")

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.S3BlobProvider{}, provider)
	})

	t.Run("Multitenant Mode Missing S3 Bucket", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_MULTITENANT", "true")

		provider, err := mcp.NewBlobProxy(ctx)
		assert.Error(t, err)
		assert.Nil(t, provider)
		assert.Contains(t, err.Error(), "OHC_S3_BUCKET must be set")
	})

	t.Run("Default to Local if Multitenant is false and Standalone is false", func(t *testing.T) {
		defer restoreEnv()

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.LocalBlobProvider{}, provider)
	})
}

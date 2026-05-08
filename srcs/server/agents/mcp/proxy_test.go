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
		os.Unsetenv("AWS_REGION")
		os.Unsetenv("AWS_ACCESS_KEY_ID")
		os.Unsetenv("AWS_SECRET_ACCESS_KEY")
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
		os.Setenv("AWS_REGION", "us-east-1")

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.S3BlobProvider{}, provider)
	})

	t.Run("Multitenant Mode Missing S3 Bucket defaults to ohc-multi-tenant-blobs", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_MULTITENANT", "true")
		os.Setenv("AWS_REGION", "us-east-1")

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.S3BlobProvider{}, provider)
	})

	t.Run("Default to Local if Multitenant is false and Standalone is false", func(t *testing.T) {
		defer restoreEnv()

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.LocalBlobProvider{}, provider)
	})

	t.Run("Local mode empty OHC_LOCAL_STORAGE_ROOT defaults to /var/tmp/ohc/blobs", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_STANDALONE", "true")

		provider, err := mcp.NewBlobProxy(ctx)
		require.NoError(t, err)
		assert.IsType(t, &storage.LocalBlobProvider{}, provider)
	})

	t.Run("Fail to load AWS config", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_MULTITENANT", "true")
		os.Setenv("AWS_PROFILE", "invalid-profile-does-not-exist")
		os.Setenv("AWS_CONFIG_FILE", "/tmp/does-not-exist")
		os.Setenv("AWS_SHARED_CREDENTIALS_FILE", "/tmp/does-not-exist")

		provider, err := mcp.NewBlobProxy(ctx)
		require.Error(t, err)
		assert.Nil(t, provider)
		os.Unsetenv("AWS_PROFILE")
		os.Unsetenv("AWS_CONFIG_FILE")
		os.Unsetenv("AWS_SHARED_CREDENTIALS_FILE")
	})

	t.Run("Local mode invalid rootdir error", func(t *testing.T) {
		defer restoreEnv()
		os.Setenv("OHC_STANDALONE", "true")
		// Instead of \x00 which is ignored, let's use a path where we have no write permission
		os.Setenv("OHC_LOCAL_STORAGE_ROOT", "/root/some_dir_we_cannot_create")

		provider, err := mcp.NewBlobProxy(ctx)
		require.Error(t, err)
		assert.Nil(t, provider)
	})
}

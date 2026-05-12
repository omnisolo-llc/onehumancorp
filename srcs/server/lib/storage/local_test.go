package storage_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"onehumancorp/srcs/server/lib/storage"
)

func TestLocalBlobProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := storage.NewLocalBlobProvider(tempDir)
	require.NoError(t, err)

	ctx := context.Background()
	testPath := "some/test/path.txt"
	testData := []byte("hello world")

	err = provider.WriteBlob(ctx, testPath, testData)
	require.NoError(t, err)

	readData, err := provider.ReadBlob(ctx, testPath)
	require.NoError(t, err)
	assert.Equal(t, testData, readData)

	// Test escaping sandbox
	err = provider.WriteBlob(ctx, "../escape.txt", testData)
	require.Error(t, err)
	assert.Contains(t, err.Error(), "escapes sandbox")

	_, err = provider.ReadBlob(ctx, "../escape.txt")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "escapes sandbox")

	// Test read non-existent
	_, err = provider.ReadBlob(ctx, "does/not/exist.txt")
	require.Error(t, err)
}

func TestNewLocalBlobProvider_Error(t *testing.T) {
	// Root dir cannot be created (no permissions)
	provider, err := storage.NewLocalBlobProvider("/root/some_dir")
	require.Error(t, err)
	assert.Nil(t, provider)
}

func TestLocalBlobProvider_WriteBlob_Error(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := storage.NewLocalBlobProvider(tempDir)
	require.NoError(t, err)

	ctx := context.Background()

	// Make root read-only
	os.Chmod(tempDir, 0555)

	err = provider.WriteBlob(ctx, "test.txt", []byte("data"))
	require.Error(t, err)

	os.Chmod(tempDir, 0755)

	// Make a file where a directory needs to be
	err = os.WriteFile(filepath.Join(tempDir, "file_as_dir"), []byte("data"), 0644)
	require.NoError(t, err)

	err = provider.WriteBlob(ctx, "file_as_dir/test.txt", []byte("data"))
	require.Error(t, err)
}

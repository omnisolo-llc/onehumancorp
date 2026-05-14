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
	path := "test-folder/test-file.txt"
	data := []byte("hello local storage")

	// Test Write
	err = provider.WriteBlob(ctx, path, data)
	assert.NoError(t, err)

	// Verify file is actually on disk
	writtenData, err := os.ReadFile(filepath.Join(tempDir, path))
	assert.NoError(t, err)
	assert.Equal(t, data, writtenData)

	// Test Read
	readData, err := provider.ReadBlob(ctx, path)
	assert.NoError(t, err)
	assert.Equal(t, data, readData)

	// Test Sandbox Escape
	err = provider.WriteBlob(ctx, "../escape.txt", []byte("bad"))
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "path escapes sandbox")
}

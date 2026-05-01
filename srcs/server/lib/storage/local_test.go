package storage

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalBlobProvider(t *testing.T) {
	// Create a temporary directory for testing
	tmpDir, err := os.MkdirTemp("", "local_blob_provider_test")
	require.NoError(t, err)
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalBlobProvider(tmpDir)
	require.NoError(t, err)

	ctx := context.Background()
	testPath := "test/file.txt"
	testData := []byte("hello world")

	// Test writing a blob
	err = provider.WriteBlob(ctx, testPath, testData)
	assert.NoError(t, err)

	// Verify file exists on disk
	fullPath := filepath.Join(tmpDir, testPath)
	_, err = os.Stat(fullPath)
	assert.NoError(t, err)

	// Test reading a blob
	readData, err := provider.ReadBlob(ctx, testPath)
	assert.NoError(t, err)
	assert.Equal(t, testData, readData)

	// Test path security (prevent path traversal)
	badPath := "../outside.txt"
	err = provider.WriteBlob(ctx, badPath, testData)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "is outside base directory")

	_, err = provider.ReadBlob(ctx, badPath)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "is outside base directory")

	// Test path security for sibling directories (sandbox escape)
	siblingPath := "../local_blob_provider_test_secrets/file.txt"
	err = provider.WriteBlob(ctx, siblingPath, testData)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "is outside base directory")

	_, err = provider.ReadBlob(ctx, siblingPath)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "is outside base directory")

	// Test reading non-existent file
	_, err = provider.ReadBlob(ctx, "nonexistent.txt")
	assert.Error(t, err)
}

func TestNewLocalBlobProvider_InvalidBaseDir(t *testing.T) {
    // Attempt to create a provider with an invalid base directory (e.g. a file)
    tmpFile, err := os.CreateTemp("", "file_instead_of_dir")
    require.NoError(t, err)
    tmpFile.Close()
    defer os.Remove(tmpFile.Name())

    // It should fail to create the base directory
    _, err = NewLocalBlobProvider(tmpFile.Name())
    assert.Error(t, err)
    assert.Contains(t, err.Error(), "failed to create base directory")
}

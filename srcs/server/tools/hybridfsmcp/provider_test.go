package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)

	ctx := context.Background()

	// Valid path
	err := provider.WriteFile(ctx, "valid.txt", []byte("test"))
	assert.NoError(t, err)

	// Traversal attempt
	err = provider.WriteFile(ctx, "../escape.txt", []byte("test"))
	assert.ErrorContains(t, err, "path traversal detected")

	_, err = provider.ReadFile(ctx, "../escape.txt")
	assert.ErrorContains(t, err, "path traversal detected")
}

func TestLocalFSProvider_Operations(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)

	ctx := context.Background()

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	assert.NoError(t, err)

	data, err := provider.ReadFile(ctx, "test.txt")
	assert.NoError(t, err)
	assert.Equal(t, "hello", string(data))

	infos, err := provider.ListDir(ctx, ".")
	assert.NoError(t, err)
	require.Len(t, infos, 1)
	assert.Equal(t, "test.txt", infos[0].Name)
	assert.False(t, infos[0].IsDir)
}

func TestCloudFSProvider_PathTraversal(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	ctx := context.Background()

	// Valid path
	err := provider.WriteFile(ctx, "tenant1", "valid.txt", []byte("test"))
	assert.NoError(t, err)

	// Traversal attempt
	err = provider.WriteFile(ctx, "tenant1", "../tenant2/escape.txt", []byte("test"))
	assert.ErrorContains(t, err, "path traversal detected")
}

func TestCloudFSProvider_Operations(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	ctx := context.Background()

	err := provider.WriteFile(ctx, "tenant1", "test.txt", []byte("hello"))
	assert.NoError(t, err)

	data, err := provider.ReadFile(ctx, "tenant1", "test.txt")
	assert.NoError(t, err)
	assert.Equal(t, "hello", string(data))

	// Tenant 2 should not see Tenant 1's files (it shouldn't even exist yet)
	_, err = provider.ReadFile(ctx, "tenant2", "test.txt")
	assert.ErrorIs(t, err, os.ErrNotExist)

	infos, err := provider.ListDir(ctx, "tenant1", ".")
	assert.NoError(t, err)
	require.Len(t, infos, 1)
	assert.Equal(t, "test.txt", infos[0].Name)
}

func TestCloudToFSProviderAdapter(t *testing.T) {
    tempDir := t.TempDir()
    cloudProvider := NewCloudFSProvider(tempDir)

    adapter := NewCloudToFSProviderAdapter(cloudProvider, func(ctx context.Context) string {
        return "tenant1"
    })

    ctx := context.Background()

    err := adapter.WriteFile(ctx, "test.txt", []byte("hello"))
    assert.NoError(t, err)

    data, err := adapter.ReadFile(ctx, "test.txt")
    assert.NoError(t, err)
    assert.Equal(t, "hello", string(data))

    infos, err := adapter.ListDir(ctx, ".")
    assert.NoError(t, err)
    require.Len(t, infos, 1)

    // Check actual disk structure to ensure tenant scoping worked
    _, err = os.Stat(filepath.Join(tempDir, "tenant1", "test.txt"))
    assert.NoError(t, err)
}

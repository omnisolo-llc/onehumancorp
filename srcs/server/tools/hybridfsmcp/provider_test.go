package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test_*")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	require.NoError(t, err)

	ctx := context.Background()

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello world"))
		require.NoError(t, err)

		data, err := provider.ReadFile(ctx, "test.txt")
		require.NoError(t, err)
		assert.Equal(t, "hello world", string(data))
	})

	t.Run("List Directory", func(t *testing.T) {
		err := provider.WriteFile(ctx, "subdir/test2.txt", []byte("test"))
		require.NoError(t, err)

		entries, err := provider.ListDir(ctx, "subdir")
		require.NoError(t, err)
		assert.Len(t, entries, 1)
		assert.Equal(t, "test2.txt", entries[0].Name())
		assert.False(t, entries[0].IsDir())
	})

	t.Run("Path Bounding", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../outside.txt")
		assert.ErrorIs(t, err, ErrAccessDenied)

		err = provider.WriteFile(ctx, "/etc/passwd", []byte("hacked"))
		assert.ErrorIs(t, err, ErrAccessDenied)
	})
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test_*")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	require.NoError(t, err)

	orgID := "org_123"
	claims := &auth.Claims{OrganizationID: orgID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
		require.NoError(t, err)

		data, err := provider.ReadFile(ctx, "test.txt")
		require.NoError(t, err)
		assert.Equal(t, "hello cloud", string(data))

		// Verify it was actually written to the tenant dir
		actualPath := filepath.Join(tempDir, orgID, "test.txt")
		actualData, err := os.ReadFile(actualPath)
		require.NoError(t, err)
		assert.Equal(t, "hello cloud", string(actualData))
	})

	t.Run("Unauthorized Access", func(t *testing.T) {
		ctxNoClaims := context.Background()
		_, err := provider.ReadFile(ctxNoClaims, "test.txt")
		assert.ErrorIs(t, err, ErrUnauthorized)
	})

	t.Run("Path Bounding", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../outside.txt")
		assert.ErrorIs(t, err, ErrAccessDenied)
	})
}

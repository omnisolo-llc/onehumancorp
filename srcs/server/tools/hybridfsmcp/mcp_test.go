package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
		assert.NoError(t, err)

		data, err := provider.ReadFile(ctx, nil, "test.txt")
		assert.NoError(t, err)
		assert.Equal(t, []byte("hello local"), data)
	})

	t.Run("List Directory", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "dir/file1.txt", []byte("1"))
		assert.NoError(t, err)

		entries, err := provider.ListDir(ctx, nil, "dir")
		assert.NoError(t, err)
		assert.Contains(t, entries, "file1.txt")
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "../outside.txt", []byte("hack"))
		assert.ErrorIs(t, err, ErrPathTraversal)
	})
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}
	claims2 := &auth.Claims{OrganizationID: "tenant2"}

	t.Run("Write and Read File Tenant 1", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello tenant1"))
		assert.NoError(t, err)

		data, err := provider.ReadFile(ctx, claims, "test.txt")
		assert.NoError(t, err)
		assert.Equal(t, []byte("hello tenant1"), data)
	})

	t.Run("Isolation: Tenant 2 cannot read Tenant 1 file", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, claims2, "test.txt")
		assert.Error(t, err) // Should fail to read
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "../tenant2/hack.txt", []byte("hack"))
		assert.ErrorIs(t, err, ErrPathTraversal)
	})

	t.Run("Missing Claims", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "test.txt", []byte("data"))
		assert.ErrorIs(t, err, ErrUnauthorized)
	})
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	t.Run("Call WriteFile", func(t *testing.T) {
		args := map[string]interface{}{
			"path": "mcp_test.txt",
			"data": base64.StdEncoding.EncodeToString([]byte("mcp data")),
		}
		res, err := mcp.CallTool(ctx, "write_file", args)
		assert.NoError(t, err)
		resMap := res.(map[string]interface{})
		assert.Equal(t, "success", resMap["status"])
	})

	t.Run("Call ReadFile", func(t *testing.T) {
		args := map[string]interface{}{
			"path": "mcp_test.txt",
		}
		res, err := mcp.CallTool(ctx, "read_file", args)
		assert.NoError(t, err)
		resMap := res.(map[string]interface{})
		assert.Equal(t, "success", resMap["status"])

		decoded, _ := base64.StdEncoding.DecodeString(resMap["data"].(string))
		assert.Equal(t, []byte("mcp data"), decoded)
	})

	t.Run("Call ListDir", func(t *testing.T) {
		args := map[string]interface{}{
			"path": ".",
		}
		res, err := mcp.CallTool(ctx, "list_directory", args)
		assert.NoError(t, err)
		resMap := res.(map[string]interface{})
		assert.Equal(t, "success", resMap["status"])

		entries := resMap["entries"].([]string)
		assert.Contains(t, entries, "mcp_test.txt")
	})
}

func TestNewProvider(t *testing.T) {
	tempDir := "/tmp"
	local := NewProvider(true, tempDir)
	_, isLocal := local.(*LocalFSProvider)
	assert.True(t, isLocal)

	cloud := NewProvider(false, tempDir)
	_, isCloud := cloud.(*CloudFSProvider)
	assert.True(t, isCloud)
}

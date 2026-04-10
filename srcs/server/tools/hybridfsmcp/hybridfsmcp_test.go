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
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	require.NoError(t, err)

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	require.NoError(t, err)
	assert.Equal(t, "hello world", string(data))

	// Test ListDir
	err = provider.WriteFile(ctx, "dir1/file1.txt", []byte("file1"))
	require.NoError(t, err)

	entries, err := provider.ListDir(ctx, ".")
	require.NoError(t, err)

	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	assert.Contains(t, names, "test.txt")
	assert.Contains(t, names, "dir1")

	// Test path traversal protection
	err = provider.WriteFile(ctx, "../traversal.txt", []byte("bad"))
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "path traversal denied")
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	orgID := "tenant123"
	claims := &auth.Claims{OrganizationID: orgID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Without claims, it should fail
	ctxNoClaims := context.Background()
	err := provider.WriteFile(ctxNoClaims, "test.txt", []byte("bad"))
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unauthorized")

	// Test WriteFile with claims
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	require.NoError(t, err)

	// Verify it was written to the correct tenant directory
	tenantDir := filepath.Join(tempDir, orgID)
	data, err := os.ReadFile(filepath.Join(tenantDir, "test.txt"))
	require.NoError(t, err)
	assert.Equal(t, "hello cloud", string(data))

	// Test ReadFile
	data, err = provider.ReadFile(ctx, "test.txt")
	require.NoError(t, err)
	assert.Equal(t, "hello cloud", string(data))

	// Test ListDir
	err = provider.WriteFile(ctx, "subdir/file.txt", []byte("sub"))
	require.NoError(t, err)

	entries, err := provider.ListDir(ctx, "subdir")
	require.NoError(t, err)
	assert.Len(t, entries, 1)
	assert.Equal(t, "file.txt", entries[0].Name())

	// Test path traversal protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "path traversal denied")
}

func TestNewFileSystemProvider(t *testing.T) {
	tempDir := t.TempDir()

	p1 := NewFileSystemProvider("OHC_MULTITENANT", tempDir)
	_, isCloud := p1.(*CloudFSProvider)
	assert.True(t, isCloud)

	p2 := NewFileSystemProvider("OHC_STANDALONE", tempDir)
	_, isLocal := p2.(*LocalFSProvider)
	assert.True(t, isLocal)
}

func TestHybridFS(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewHybridFS(provider)

	tools := server.ListTools()
	assert.Len(t, tools, 3)

	ctx := context.Background()

	// Test write_file tool
	writeArgs := map[string]interface{}{"path": "test.txt", "content": "file content"}
	writeResRaw, err := server.CallTool(ctx, "write_file", writeArgs)
	require.NoError(t, err)
	writeRes, ok := writeResRaw.(map[string]interface{})
	require.True(t, ok)
	assert.True(t, writeRes["success"].(bool))

	// Test read_file tool
	readArgs := map[string]interface{}{"path": "test.txt"}
	readResRaw, err := server.CallTool(ctx, "read_file", readArgs)
	require.NoError(t, err)
	readRes, ok := readResRaw.(map[string]interface{})
	require.True(t, ok)
	assert.Equal(t, "file content", readRes["content"])

	// Test list_directory tool
	listArgs := map[string]interface{}{"path": "."}
	listResRaw, err := server.CallTool(ctx, "list_directory", listArgs)
	require.NoError(t, err)
	listRes, ok := listResRaw.(map[string]interface{})
	require.True(t, ok)
	itemsRaw, ok := listRes["items"].([]string)
	if !ok {
		// handle possible JSON unmarshalling into interface{} array
		itemsIface, ok2 := listRes["items"].([]interface{})
		require.True(t, ok2)
		for _, item := range itemsIface {
			itemsRaw = append(itemsRaw, item.(string))
		}
	}
	assert.Contains(t, itemsRaw, "test.txt")
}

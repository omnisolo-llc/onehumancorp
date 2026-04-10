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
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	require.NoError(t, err)

	data, err := provider.ReadFile(ctx, "test.txt")
	require.NoError(t, err)
	assert.Equal(t, "hello", string(data))

	files, err := provider.ListDir(ctx, ".")
	require.NoError(t, err)
	assert.Contains(t, files, "test.txt")

	searchRes, err := provider.SearchFiles(ctx, "test")
	require.NoError(t, err)
	assert.Contains(t, searchRes, "test.txt")

	// Reject absolute paths
	err = provider.WriteFile(ctx, "/absolute/path.txt", []byte("bad"))
	assert.Error(t, err)

	// Reject escaping paths
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	assert.Error(t, err)
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-1",
	})

	err := provider.WriteFile(ctx, "data.txt", []byte("cloud"))
	require.NoError(t, err)

	data, err := provider.ReadFile(ctx, "data.txt")
	require.NoError(t, err)
	assert.Equal(t, "cloud", string(data))

	files, err := provider.ListDir(ctx, ".")
	require.NoError(t, err)
	assert.Contains(t, files, "data.txt")

	searchRes, err := provider.SearchFiles(ctx, "data")
	require.NoError(t, err)
	assert.Contains(t, searchRes, "data.txt")

	// Missing claims
	_, err = provider.ReadFile(context.Background(), "data.txt")
	assert.Error(t, err)
}

func TestHybridFSMCP(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tmpDir := t.TempDir()
	server := NewHybridFSServer(tmpDir)

	tools := server.ListTools()
	assert.Len(t, tools, 4)

	ctx := context.Background()

	_, err := server.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "test content"})
	require.NoError(t, err)

	res, err := server.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	require.NoError(t, err)
	assert.Equal(t, "test content", res.(map[string]interface{})["content"])

	resList, err := server.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	require.NoError(t, err)
	assert.Contains(t, resList.(map[string]interface{})["files"].([]string), "test.txt")

	resSearch, err := server.CallTool(ctx, "search_files", map[string]interface{}{"query": "test"})
	require.NoError(t, err)
	assert.Contains(t, resSearch.(map[string]interface{})["files"].([]string), "test.txt")

	_, err = server.CallTool(ctx, "unknown", nil)
	assert.Error(t, err)
}

func TestCloudFSMCP_Multitenant(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	tmpDir := t.TempDir()
	server := NewHybridFSServer(tmpDir)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-2",
	})

	_, err := server.CallTool(ctx, "write_file", map[string]interface{}{"path": "test2.txt", "content": "test content 2"})
	require.NoError(t, err)

	data, err := os.ReadFile(filepath.Join(tmpDir, "tenant-2", "test2.txt"))
	require.NoError(t, err)
	assert.Equal(t, "test content 2", string(data))
}

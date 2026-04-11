package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestServer_ListTools(t *testing.T) {
	provider := NewLocalFSProvider(t.TempDir())
	server := NewServer(provider)

	tools := server.ListTools()
	require.Len(t, tools, 3)

	names := []string{}
	for _, tool := range tools {
		names = append(names, tool.Name)
	}
	assert.Contains(t, names, "read_file")
	assert.Contains(t, names, "write_file")
	assert.Contains(t, names, "list_directory")
}

func TestServer_CallTool_Local(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	// Write file
	res, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello mcp",
	})
	assert.NoError(t, err)
	resMap, ok := res.(map[string]interface{})
	require.True(t, ok)
	assert.Equal(t, "success", resMap["status"])
	assert.Equal(t, "standalone", resMap["mode"])

	// Read file
	res, err = server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	assert.NoError(t, err)
	resMap, ok = res.(map[string]interface{})
	require.True(t, ok)
	assert.Equal(t, "hello mcp", resMap["data"])

	// List directory
	res, err = server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	assert.NoError(t, err)
	resMap, ok = res.(map[string]interface{})
	require.True(t, ok)
	results, ok := resMap["results"].([]map[string]interface{})
	require.True(t, ok)
	require.Len(t, results, 1)
	assert.Equal(t, "test.txt", results[0]["name"])
}

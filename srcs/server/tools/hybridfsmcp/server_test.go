package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestHybridFSMCP_Factory(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_factory_test_*")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	t.Run("Standalone Mode", func(t *testing.T) {
		os.Setenv("OHC_STANDALONE", "true")
		defer os.Unsetenv("OHC_STANDALONE")

		mcp, err := NewHybridFSMCP(tempDir)
		require.NoError(t, err)
		_, ok := mcp.provider.(*LocalFSProvider)
		assert.True(t, ok, "Expected LocalFSProvider in standalone mode")
	})

	t.Run("Cloud Mode", func(t *testing.T) {
		os.Setenv("OHC_STANDALONE", "false")
		defer os.Unsetenv("OHC_STANDALONE")

		mcp, err := NewHybridFSMCP(tempDir)
		require.NoError(t, err)
		_, ok := mcp.provider.(*CloudFSProvider)
		assert.True(t, ok, "Expected CloudFSProvider in cloud mode")
	})
}

func TestHybridFSMCP_Tools(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_tools_test_*")
	require.NoError(t, err)
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp, err := NewHybridFSMCP(tempDir)
	require.NoError(t, err)

	ctx := context.Background()

	t.Run("ListTools", func(t *testing.T) {
		tools := mcp.ListTools()
		assert.Len(t, tools, 3)
		assert.Equal(t, "read_file", tools[0].Name)
		assert.Equal(t, "write_file", tools[1].Name)
		assert.Equal(t, "list_directory", tools[2].Name)
	})

	t.Run("Write and Read File via Tool", func(t *testing.T) {
		// Write
		writeInput := `{"path": "mcp_test.txt", "content": "mcp works"}`
		writeRes, err := mcp.CallTool(ctx, "write_file", []byte(writeInput))
		require.NoError(t, err)
		assert.JSONEq(t, `{"status": "success"}`, string(writeRes))

		// Read
		readInput := `{"path": "mcp_test.txt"}`
		readRes, err := mcp.CallTool(ctx, "read_file", []byte(readInput))
		require.NoError(t, err)
		assert.JSONEq(t, `{"content": "mcp works"}`, string(readRes))
	})

	t.Run("List Directory via Tool", func(t *testing.T) {
		listInput := `{"path": "."}`
		listRes, err := mcp.CallTool(ctx, "list_directory", []byte(listInput))
		require.NoError(t, err)

		var resMap map[string]interface{}
		err = json.Unmarshal(listRes, &resMap)
		require.NoError(t, err)

		entries, ok := resMap["entries"].([]interface{})
		require.True(t, ok)
		assert.Len(t, entries, 1) // Only mcp_test.txt from previous test
	})

	t.Run("Unknown Tool", func(t *testing.T) {
		_, err := mcp.CallTool(ctx, "unknown_tool", []byte(`{}`))
		assert.Error(t, err)
	})
}

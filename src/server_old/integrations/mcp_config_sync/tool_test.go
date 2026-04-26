package mcp_config_sync

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockProvider struct {
	db.Provider
}

func TestConfigTool_GetConfigTool(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "standalone")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.yaml")
	err := os.WriteFile(configPath, []byte(`my_key: local_value`), 0644)
	require.NoError(t, err)

	os.Setenv("OHC_CONFIG_PATH", configPath)
	defer os.Unsetenv("OHC_CONFIG_PATH")

	tool := NewConfigTool(nil)
	mcpTool := tool.GetConfigTool()

    require.NotNil(t, mcpTool)
    assert.Equal(t, "get_config", mcpTool.Name)
}

func TestConfigTool_SyncConfigToCloudTool(t *testing.T) {
	provider := &mockProvider{}
	tool := NewConfigTool(provider)
	mcpTool := tool.SyncConfigToCloudTool()

    require.NotNil(t, mcpTool)
    assert.Equal(t, "sync_config_to_cloud", mcpTool.Name)
}

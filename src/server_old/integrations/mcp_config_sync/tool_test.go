package mcp_config_sync

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// Mock objects for testing
type mockProvider struct {
	db.Provider
	execErr     error
	queryRowVal string
	queryRowErr error
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, m.execErr
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return &mockRow{val: m.queryRowVal, err: m.queryRowErr}
}

type mockRow struct {
	val string
	err error
}

func (m *mockRow) Scan(dest ...any) error {
	if m.err != nil {
		return m.err
	}
	if len(dest) > 0 {
		if p, ok := dest[0].(*string); ok {
			*p = m.val
		}
	}
	return nil
}

func contextWithClaims(ctx context.Context, claims *auth.Claims) context.Context {
	return context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)
}

func TestConfigTool_GetConfig_Standalone_Success(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "standalone")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.yaml")
	err := os.WriteFile(configPath, []byte(`my_key: local_value`), 0644)
	require.NoError(t, err)

	os.Setenv("OHC_CONFIG_PATH", configPath)
	defer os.Unsetenv("OHC_CONFIG_PATH")

	tool := NewConfigTool(nil)
	val, err := tool.GetConfig(context.Background(), "my_key")
	require.NoError(t, err)
	assert.Equal(t, "local_value", val)
}

func TestConfigTool_GetConfig_Standalone_NotFound(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "standalone")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.yaml")
	os.Setenv("OHC_CONFIG_PATH", configPath)
	defer os.Unsetenv("OHC_CONFIG_PATH")

	tool := NewConfigTool(nil)
	_, err := tool.GetConfig(context.Background(), "my_key")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "config file not found")
}

func TestConfigTool_GetConfig_Cloud_NoAuth(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "cloud")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tool := NewConfigTool(nil)
	_, err := tool.GetConfig(context.Background(), "some_key")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "unauthorized")
}

func TestConfigTool_GetConfig_Cloud_Success(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "cloud")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	provider := &mockProvider{queryRowVal: "cloud_value"}

	tool := NewConfigTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	val, err := tool.GetConfig(ctx, "my_key")
	require.NoError(t, err)
	assert.Equal(t, "cloud_value", val)
}

func TestConfigTool_SyncConfigToCloud_NoAuth(t *testing.T) {
	tool := NewConfigTool(nil)
	err := tool.SyncConfigToCloud(context.Background(), ConfigSyncPayload{})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "unauthorized")
}

func TestConfigTool_SyncConfigToCloud_MismatchTenant(t *testing.T) {
	tool := NewConfigTool(nil)
	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	err := tool.SyncConfigToCloud(ctx, ConfigSyncPayload{TenantID: "org2"})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "tenant ID mismatch")
}

func TestConfigTool_SyncConfigToCloud_Success(t *testing.T) {
	provider := &mockProvider{}

	tool := NewConfigTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	err := tool.SyncConfigToCloud(ctx, ConfigSyncPayload{
		TenantID: "org1",
		AgentID:  "agent1",
		Key:      "my_key",
		Value:    "my_val",
		Metadata: map[string]string{"source": "local"},
	})
	require.NoError(t, err)
}

func TestConfigTool_GetConfig_Standalone_InvalidYAML(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "standalone")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.yaml")
	err := os.WriteFile(configPath, []byte(`invalid: [ yaml: content`), 0644)
	require.NoError(t, err)

	os.Setenv("OHC_CONFIG_PATH", configPath)
	defer os.Unsetenv("OHC_CONFIG_PATH")

	tool := NewConfigTool(nil)
	_, err = tool.GetConfig(context.Background(), "my_key")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to parse config file")
}

func TestConfigTool_GetConfig_Standalone_KeyNotFound(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "standalone")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.yaml")
	err := os.WriteFile(configPath, []byte(`other_key: val`), 0644)
	require.NoError(t, err)

	os.Setenv("OHC_CONFIG_PATH", configPath)
	defer os.Unsetenv("OHC_CONFIG_PATH")

	tool := NewConfigTool(nil)
	_, err = tool.GetConfig(context.Background(), "my_key")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "key my_key not found")
}

func TestConfigTool_GetConfig_Cloud_QueryError(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "cloud")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	provider := &mockProvider{queryRowErr: os.ErrPermission}
	tool := NewConfigTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	_, err := tool.GetConfig(ctx, "my_key")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to get config")
}

func TestConfigTool_SyncConfigToCloud_ExecError(t *testing.T) {
	provider := &mockProvider{execErr: os.ErrPermission}
	tool := NewConfigTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	err := tool.SyncConfigToCloud(ctx, ConfigSyncPayload{
		TenantID: "org1",
		AgentID:  "agent1",
		Key:      "my_key",
		Value:    "my_val",
	})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to sync config to cloud")
}

func TestConfigTool_GetConfig_Standalone_ReadError(t *testing.T) {
	os.Setenv("OHC_HYBRID_MODE", "standalone")
	defer os.Unsetenv("OHC_HYBRID_MODE")

	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.yaml")
	err := os.Mkdir(configPath, 0755)
	require.NoError(t, err)

	os.Setenv("OHC_CONFIG_PATH", configPath)
	defer os.Unsetenv("OHC_CONFIG_PATH")

	tool := NewConfigTool(nil)
	_, err = tool.GetConfig(context.Background(), "my_key")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to read config file")
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

	// test invalid JSON
	_, err = mcpTool.Execute(context.Background(), "", map[string]interface{}{"invalid": "json"})
	require.Error(t, err)

	// test missing key
	_, err = mcpTool.Execute(context.Background(), "", map[string]interface{}{})
	require.Error(t, err)

	// test success
	res, err := mcpTool.Execute(context.Background(), "", map[string]interface{}{"key": "my_key"})
	require.NoError(t, err)
	assert.Equal(t, "local_value", res)

    // test execution error (key not found)
    _, err = mcpTool.Execute(context.Background(), "", map[string]interface{}{"key": "other_key"})
	require.Error(t, err)
}

func TestConfigTool_SyncConfigToCloudTool(t *testing.T) {
	provider := &mockProvider{}
	tool := NewConfigTool(provider)
	mcpTool := tool.SyncConfigToCloudTool()

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})

	// test invalid JSON
	_, err := mcpTool.Execute(ctx, "", map[string]interface{}{"invalid": "json"})
	require.Error(t, err)

	// test missing fields
	_, err = mcpTool.Execute(ctx, "", map[string]interface{}{"tenant_id":"org1"})
	require.Error(t, err)

	// test success
	res, err := mcpTool.Execute(ctx, "", map[string]interface{}{"tenant_id":"org1","agent_id":"a1","key":"k1","value":"v1","metadata":map[string]interface{}{"k":"v"}})
	require.NoError(t, err)
	assert.Equal(t, "Successfully synced config to cloud", res)

    // test execution error (no auth)
    _, err = mcpTool.Execute(context.Background(), "", map[string]interface{}{"tenant_id":"org1","agent_id":"a1","key":"k1","value":"v1","metadata":map[string]interface{}{"k":"v"}})
	require.Error(t, err)
}

package mcp_config_sync_test

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/integrations/mcp_config_sync"
)

func TestLocalConfigProvider(t *testing.T) {
	provider := db.NewTestProvider(t)
	localProvider := mcp_config_sync.NewLocalConfigProvider(provider)

	ctx := context.Background()

	// Test GetConfig
	payload, err := localProvider.GetConfig(ctx, "test_key")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if payload.Key != "test_key" {
		t.Errorf("expected key test_key, got %s", payload.Key)
	}

	// Test SyncConfigToCloud
	err = localProvider.SyncConfigToCloud(ctx, mcp_config_sync.ConfigSyncPayload{})
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestCloudConfigProvider(t *testing.T) {
	provider := db.NewTestProvider(t)
	// Setup test database schema
	_, err := provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			tenant_id VARCHAR(255) NOT NULL,
			agent_id VARCHAR(255),
			key VARCHAR(255) NOT NULL,
			value TEXT NOT NULL,
			metadata TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	cloudProvider := mcp_config_sync.NewCloudConfigProvider(provider)

	ctx := context.Background()

	// Test Unauthorized Get
	_, err = cloudProvider.GetConfig(ctx, "k")
	if err == nil {
		t.Errorf("expected error for unauthorized GetConfig")
	}

	// Test Unauthorized Sync
	err = cloudProvider.SyncConfigToCloud(ctx, mcp_config_sync.ConfigSyncPayload{Key: "k", Value: "v"})
	if err == nil || err.Error() != "unauthorized: missing claims or organization ID" {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	// Setup Authorized Context
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test GetConfig Not Found
	_, err = cloudProvider.GetConfig(ctx, "not_found")
	if err == nil {
		t.Errorf("expected error for missing key")
	}

	// Test Authorized Sync
	err = cloudProvider.SyncConfigToCloud(ctx, mcp_config_sync.ConfigSyncPayload{Key: "my_key", Value: "my_value", AgentID: "agent-1", Metadata: map[string]string{"env": "prod"}})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test Authorized Sync with json error (simulating marshaling error is hard without custom types, but testing branch)
	err = cloudProvider.SyncConfigToCloud(ctx, mcp_config_sync.ConfigSyncPayload{Key: "my_key2", Value: "my_value2", AgentID: "agent-1", Metadata: nil})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test Authorized Get
	payload, err := cloudProvider.GetConfig(ctx, "my_key")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if payload.Value != "my_value" {
		t.Errorf("expected my_value, got %s", payload.Value)
	}
	if payload.AgentID != "agent-1" {
		t.Errorf("expected agent-1, got %s", payload.AgentID)
	}
	if payload.Metadata["env"] != "prod" {
		t.Errorf("expected prod env, got %s", payload.Metadata["env"])
	}

	// Test Authorized Get with empty metadata
	err = cloudProvider.SyncConfigToCloud(ctx, mcp_config_sync.ConfigSyncPayload{Key: "my_key2", Value: "my_value2", AgentID: "agent-1", Metadata: nil})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	_, _ = cloudProvider.GetConfig(ctx, "my_key2")

	// Test Authorized Get json unmarshal error string
	_, _ = provider.Exec(context.Background(), "INSERT INTO mcp_config_sync_log(tenant_id, agent_id, key, value, metadata) VALUES('tenant-1', 'agent-1', 'bad_json', 'val', '{badjson}')")
	_, _ = cloudProvider.GetConfig(ctx, "bad_json")

	// Drop table to test DB error on sync
	_, _ = provider.Exec(context.Background(), "DROP TABLE mcp_config_sync_log")
	err = cloudProvider.SyncConfigToCloud(ctx, mcp_config_sync.ConfigSyncPayload{Key: "k", Value: "v"})
	if err == nil {
		t.Errorf("expected db error")
	}
}

func TestConfigSyncMCP(t *testing.T) {
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			tenant_id VARCHAR(255) NOT NULL,
			agent_id VARCHAR(255),
			key VARCHAR(255) NOT NULL,
			value TEXT NOT NULL,
			metadata TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	cloudProvider := mcp_config_sync.NewCloudConfigProvider(provider)
	mcp := mcp_config_sync.NewConfigSyncMCP(cloudProvider)

	tools := mcp.ListTools()
	if len(tools) != 2 {
		t.Errorf("expected 2 tools, got %d", len(tools))
	}

	ctx := context.Background()

    // Test get_config error
    _, err = mcp.CallTool(ctx, "get_config", map[string]interface{}{"key": "test"})
	if err == nil {
		t.Errorf("expected error")
	}

    claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    // Test successful sync_config_to_cloud
	res, err := mcp.CallTool(ctx, "sync_config_to_cloud", map[string]interface{}{
		"key": "test",
		"value": "val",
		"agent_id": "a1",
		"metadata": map[string]interface{}{
			"env": "prod",
		},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
    if resMap, ok := res.(map[string]interface{}); ok {
        if resMap["status"] != "success" {
            t.Errorf("expected success")
        }
    } else {
        t.Errorf("expected map")
    }

	// Test get_config
	res, err = mcp.CallTool(ctx, "get_config", map[string]interface{}{"key": "test"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); ok {
		if config, ok := resMap["config"].(*mcp_config_sync.ConfigSyncPayload); ok {
			if config.Key != "test" {
				t.Errorf("expected key test, got %s", config.Key)
			}
		} else {
			t.Errorf("expected config payload in response")
		}
	} else {
		t.Errorf("expected map response")
	}

	// Test get_config missing key
	_, err = mcp.CallTool(ctx, "get_config", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for missing key in get_config")
	}

    // Cover the branch where metadata is not a map
    _, err = mcp.CallTool(ctx, "sync_config_to_cloud", map[string]interface{}{
		"key": "test",
		"value": "val",
		"agent_id": "a1",
		"metadata": "not_a_map",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test sync_config_to_cloud missing args
	_, err = mcp.CallTool(ctx, "sync_config_to_cloud", map[string]interface{}{"value": "val"})
	if err == nil {
		t.Errorf("expected error for missing key in sync")
	}

	_, err = mcp.CallTool(ctx, "sync_config_to_cloud", map[string]interface{}{"key": "test"})
	if err == nil {
		t.Errorf("expected error for missing value in sync")
	}

	// Test get_config provider error
	errProvider := mcp_config_sync.NewCloudConfigProvider(provider) // Will error without context
	errMcp := mcp_config_sync.NewConfigSyncMCP(errProvider)
	_, err = errMcp.CallTool(context.Background(), "get_config", map[string]interface{}{"key": "test"})
	if err == nil {
		t.Errorf("expected error from provider")
	}

	// Test sync_config_to_cloud provider error
	_, err = errMcp.CallTool(context.Background(), "sync_config_to_cloud", map[string]interface{}{"key": "test", "value": "val"})
	if err == nil {
		t.Errorf("expected error from provider sync")
	}

	// Test unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("expected error for unknown tool")
	}
}

func TestFactory(t *testing.T) {
	provider := db.NewTestProvider(t)

	// Test Cloud (default or env=true)
	os.Setenv("OHC_MULTITENANT", "true")
	p := mcp_config_sync.NewProviderFactory(provider)
	if _, ok := p.(*mcp_config_sync.CloudConfigProvider); !ok {
		t.Errorf("expected CloudConfigProvider")
	}

	// Test Local (env=false)
	os.Setenv("OHC_MULTITENANT", "false")
	p = mcp_config_sync.NewProviderFactory(provider)
	if _, ok := p.(*mcp_config_sync.LocalConfigProvider); !ok {
		t.Errorf("expected LocalConfigProvider")
	}

	os.Setenv("OHC_MULTITENANT", "")
	p = mcp_config_sync.NewProviderFactory(provider)
	if _, ok := p.(*mcp_config_sync.CloudConfigProvider); !ok {
		t.Errorf("expected CloudConfigProvider for default")
	}
}

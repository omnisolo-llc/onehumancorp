package statesyncmcp

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
	isSQLite bool
}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	return 0, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	return nil
}

func (m *mockProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	return nil, nil
}

func (m *mockProvider) Close() {
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil
}

func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

func TestListTools(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true})
	mcp := NewStateSyncMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	expectedTools := map[string]bool{
		"sync_local_to_cloud": true,
		"sync_cloud_to_local": true,
		"get_sync_status":     true,
	}

	for _, tool := range tools {
		if !expectedTools[tool.Name] {
			t.Errorf("unexpected tool: %s", tool.Name)
		}
	}
}

func TestCallToolUnauthorized(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true})
	mcp := NewStateSyncMCP(provider)

	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatal("expected unauthorized error")
	}
	if err.Error() != "unauthorized: missing claims" {
		t.Errorf("unexpected error: %s", err)
	}
}

func TestCallToolUnknownTool(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Fatal("expected unknown tool error")
	}
}

func TestCallToolSyncLocalToCloudSQLite(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %s", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if m["status"] != "success" {
		t.Errorf("expected status success, got %v", m["status"])
	}
	if m["message"] != "mock sync up complete" {
		t.Errorf("expected message 'mock sync up complete', got %v", m["message"])
	}
}

func TestCallToolSyncLocalToCloudPostgres(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: false})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %s", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if m["status"] != "success" {
		t.Errorf("expected status success, got %v", m["status"])
	}
	if m["message"] != "no-op in cloud mode" {
		t.Errorf("expected message 'no-op in cloud mode', got %v", m["message"])
	}
}

func TestCallToolSyncCloudToLocalSQLite(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %s", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if m["status"] != "success" {
		t.Errorf("expected status success, got %v", m["status"])
	}
	if m["message"] != "mock sync down complete" {
		t.Errorf("expected message 'mock sync down complete', got %v", m["message"])
	}
}

func TestCallToolSyncCloudToLocalPostgres(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: false})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %s", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if m["status"] != "success" {
		t.Errorf("expected status success, got %v", m["status"])
	}
	if m["message"] != "no-op in cloud mode" {
		t.Errorf("expected message 'no-op in cloud mode', got %v", m["message"])
	}
}

func TestCallToolGetStatusSQLite(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: true})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %s", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if m["status"] != "success" {
		t.Errorf("expected status success, got %v", m["status"])
	}
	if m["mode"] != "standalone" {
		t.Errorf("expected mode standalone, got %v", m["mode"])
	}
	if _, err := time.Parse(time.RFC3339, m["last_sync"].(string)); err != nil {
		t.Errorf("expected valid RFC3339 timestamp, got %v", m["last_sync"])
	}
}

func TestCallToolGetStatusPostgres(t *testing.T) {
	provider := NewDefaultStateSyncProvider(&mockProvider{isSQLite: false})
	mcp := NewStateSyncMCP(provider)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	res, err := mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %s", err)
	}

	m, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("expected map response")
	}
	if m["status"] != "success" {
		t.Errorf("expected status success, got %v", m["status"])
	}
	if m["mode"] != "cloud" {
		t.Errorf("expected mode cloud, got %v", m["mode"])
	}
}

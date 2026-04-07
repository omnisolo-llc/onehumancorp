package statesyncmcp

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockDBProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

type mockSyncProvider struct {
	syncUpErr   error
	syncDownErr error
	statusData  map[string]interface{}
	statusErr   error
}

func (m *mockSyncProvider) SyncUp(ctx context.Context, claims *auth.Claims) error {
	return m.syncUpErr
}

func (m *mockSyncProvider) SyncDown(ctx context.Context, claims *auth.Claims) error {
	return m.syncDownErr
}

func (m *mockSyncProvider) GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return m.statusData, m.statusErr
}

func TestStateSyncMCP_ListTools(t *testing.T) {
	mcp := NewStateSyncMCP(&mockDBProvider{}, &mockSyncProvider{})
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}
}

func TestStateSyncMCP_CallTool_Unauthorized(t *testing.T) {
	mcp := NewStateSyncMCP(&mockDBProvider{}, &mockSyncProvider{})
	_, err := mcp.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestStateSyncMCP_CallTool_CloudMode(t *testing.T) {
	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: false}, &mockSyncProvider{})

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{})
	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	m := res.(map[string]interface{})
	if m["status"] != "success" {
		t.Errorf("expected success status, got %v", m["status"])
	}
}

func TestStateSyncMCP_CallTool_SQLiteMode(t *testing.T) {
	syncProvider := &mockSyncProvider{
		statusData: map[string]interface{}{"last_sync": "2023-01-01"},
	}
	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: true}, syncProvider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{})

	// Test sync_local_to_cloud
	res, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	m := res.(map[string]interface{})
	if m["status"] != "success" {
		t.Errorf("expected success status, got %v", m["status"])
	}

	// Test sync_cloud_to_local
	res, err = mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	m = res.(map[string]interface{})
	if m["status"] != "success" {
		t.Errorf("expected success status, got %v", m["status"])
	}

	// Test get_sync_status
	res, err = mcp.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	m = res.(map[string]interface{})
	if m["last_sync"] != "2023-01-01" {
		t.Errorf("expected last_sync 2023-01-01, got %v", m["last_sync"])
	}

	// Test unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestStateSyncMCP_CallTool_Errors(t *testing.T) {
	syncProvider := &mockSyncProvider{
		syncUpErr: errors.New("up error"),
		syncDownErr: errors.New("down error"),
	}
	mcp := NewStateSyncMCP(&mockDBProvider{isSQLite: true}, syncProvider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{})

	_, err := mcp.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil || err.Error() != "up error" {
		t.Fatalf("expected up error, got %v", err)
	}

	_, err = mcp.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil || err.Error() != "down error" {
		t.Fatalf("expected down error, got %v", err)
	}
}

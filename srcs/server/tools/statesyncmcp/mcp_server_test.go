package statesyncmcp

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// MockProvider implements StateSyncProvider for testing.
type MockProvider struct {
	SyncUpResult    SyncResult
	SyncUpError     error
	SyncDownResult  SyncResult
	SyncDownError   error
	GetStatusResult SyncStatusResponse
	GetStatusError  error
}

func (m *MockProvider) SyncUp(ctx context.Context) (SyncResult, error) {
	return m.SyncUpResult, m.SyncUpError
}

func (m *MockProvider) SyncDown(ctx context.Context) (SyncResult, error) {
	return m.SyncDownResult, m.SyncDownError
}

func (m *MockProvider) GetStatus(ctx context.Context) (SyncStatusResponse, error) {
	return m.GetStatusResult, m.GetStatusError
}

func TestListTools(t *testing.T) {
	server := NewServer(&MockProvider{})
	tools, err := server.ListTools(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}
}

func TestCallTool_SyncLocalToCloud(t *testing.T) {
	mockProvider := &MockProvider{
		SyncUpResult: SyncResult{SyncedRecords: 5, Errors: 0},
	}
	server := NewServer(mockProvider)

	res, err := server.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var sr SyncResult
	if err := json.Unmarshal([]byte(res), &sr); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if sr.SyncedRecords != 5 {
		t.Errorf("expected 5 synced records, got %d", sr.SyncedRecords)
	}
}

func TestCallTool_SyncCloudToLocal(t *testing.T) {
	mockProvider := &MockProvider{
		SyncDownResult: SyncResult{SyncedRecords: 10, Errors: 1},
	}
	server := NewServer(mockProvider)

	res, err := server.CallTool(context.Background(), "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var sr SyncResult
	if err := json.Unmarshal([]byte(res), &sr); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if sr.SyncedRecords != 10 {
		t.Errorf("expected 10 synced records, got %d", sr.SyncedRecords)
	}
}

func TestCallTool_GetSyncStatus(t *testing.T) {
	lastSync := time.Now().Format(time.RFC3339)
	mockProvider := &MockProvider{
		GetStatusResult: SyncStatusResponse{PendingUp: 2, PendingDown: 3, LastSync: lastSync},
	}
	server := NewServer(mockProvider)

	res, err := server.CallTool(context.Background(), "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var sr SyncStatusResponse
	if err := json.Unmarshal([]byte(res), &sr); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if sr.PendingUp != 2 {
		t.Errorf("expected 2 pending up, got %d", sr.PendingUp)
	}
}

func TestCallTool_UnknownTool(t *testing.T) {
	server := NewServer(&MockProvider{})
	_, err := server.CallTool(context.Background(), "unknown_tool", nil)
	if err == nil {
		t.Fatal("expected error for unknown tool")
	}
}

func TestLocalSyncProvider_Unauthorized(t *testing.T) {
	provider := NewLocalSyncProvider()
	ctx := context.Background() // Missing claims

	_, err := provider.SyncUp(ctx)
	if err == nil {
		t.Error("expected error for missing claims in SyncUp")
	}

	_, err = provider.SyncDown(ctx)
	if err == nil {
		t.Error("expected error for missing claims in SyncDown")
	}

	_, err = provider.GetStatus(ctx)
	if err == nil {
		t.Error("expected error for missing claims in GetStatus")
	}
}

func TestLocalSyncProvider_Authorized(t *testing.T) {
	provider := NewLocalSyncProvider()
	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	resUp, err := provider.SyncUp(ctx)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if resUp.SyncedRecords != 0 {
		t.Errorf("expected 0 synced records, got %d", resUp.SyncedRecords)
	}
}

func TestCloudSyncProvider(t *testing.T) {
	provider := NewCloudSyncProvider()
	ctx := context.Background()

	_, err := provider.SyncUp(ctx)
	if err == nil {
		t.Error("expected error in CloudSyncProvider SyncUp")
	}

	_, err = provider.SyncDown(ctx)
	if err == nil {
		t.Error("expected error in CloudSyncProvider SyncDown")
	}

	res, err := provider.GetStatus(ctx)
	if err != nil {
		t.Errorf("unexpected error in CloudSyncProvider GetStatus: %v", err)
	}
	if res.LastSync != "Never" {
		t.Errorf("expected LastSync 'Never', got %s", res.LastSync)
	}
}

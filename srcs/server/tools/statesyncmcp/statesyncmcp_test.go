package statesyncmcp

import (
	"context"
	"encoding/json"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

var errMock = errors.New("mock error")

// A failing provider to test the error paths
type ErrorProvider struct{}

func (e *ErrorProvider) SyncUp(ctx context.Context, claims *auth.Claims, payload json.RawMessage) (*SyncResult, error) {
	return nil, errMock
}
func (e *ErrorProvider) SyncDown(ctx context.Context, claims *auth.Claims) (*SyncResult, error) {
	return nil, errMock
}
func (e *ErrorProvider) GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatusResponse, error) {
	return nil, errMock
}

func TestMCPServer_ListTools(t *testing.T) {
	provider := NewMockProvider()
	server := NewMCPServer(provider)

	tools, err := server.ListTools(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

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
			t.Errorf("unexpected tool name: %s", tool.Name)
		}
	}
}

func TestMCPServer_CallTool_Unauthorized(t *testing.T) {
	provider := NewMockProvider()
	server := NewMCPServer(provider)

	// Context without claims
	ctx := context.Background()

	_, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatal("expected unauthorized error, got nil")
	}

	if err.Error() != "unauthorized: missing auth claims" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestMCPServer_CallTool_UnknownTool(t *testing.T) {
	provider := NewMockProvider()
	server := NewMCPServer(provider)

	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := server.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Fatal("expected unknown tool error, got nil")
	}

	if err.Error() != "unknown tool: unknown_tool" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestMCPServer_CallTool_SyncLocalToCloud(t *testing.T) {
	provider := NewMockProvider()
	server := NewMCPServer(provider)

	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	args := json.RawMessage(`{"data": "test"}`)
	resultJSON, err := server.CallTool(ctx, "sync_local_to_cloud", args)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var result SyncResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if result.Status != SyncStatusSuccess {
		t.Errorf("expected success status, got %s", result.Status)
	}
	if result.SyncedRecords != 1 {
		t.Errorf("expected 1 synced record, got %d", result.SyncedRecords)
	}
	if time.Since(result.Timestamp) > time.Second {
		t.Errorf("unexpected timestamp: %v", result.Timestamp)
	}
}

func TestMCPServer_CallTool_SyncCloudToLocal(t *testing.T) {
	provider := NewMockProvider()
	server := NewMCPServer(provider)

	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	resultJSON, err := server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var result SyncResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if result.Status != SyncStatusSuccess {
		t.Errorf("expected success status, got %s", result.Status)
	}
	if result.SyncedRecords != 0 {
		t.Errorf("expected 0 synced records, got %d", result.SyncedRecords)
	}
}

func TestMCPServer_CallTool_GetSyncStatus(t *testing.T) {
	provider := NewMockProvider()
	server := NewMCPServer(provider)

	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	resultJSON, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var result SyncStatusResponse
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if result.Status != SyncStatusSuccess {
		t.Errorf("expected success status, got %s", result.Status)
	}
	if result.PendingCount != 0 {
		t.Errorf("expected 0 pending count, got %d", result.PendingCount)
	}
}

func TestMCPServer_CallTool_Errors(t *testing.T) {
	provider := &ErrorProvider{}
	server := NewMCPServer(provider)

	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != errMock {
		t.Fatalf("expected error, got %v", err)
	}

	_, err = server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != errMock {
		t.Fatalf("expected error, got %v", err)
	}

	_, err = server.CallTool(ctx, "get_sync_status", nil)
	if err != errMock {
		t.Fatalf("expected error, got %v", err)
	}
}

package mcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestSecretsSyncTool_Execute_ValidPull(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewSecretsSyncTool(proxy)

	keys := []string{"api_key", "db_password"}
	err := tool.Execute(ctx, keys, "pull")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if mockDB.execCalls != 2 { // One for buffer, one for sync status update
		t.Errorf("Expected 2 Exec calls, got %d", mockDB.execCalls)
	}
	if mockDB.queryCalls != 1 {
		t.Errorf("Expected 1 Query call, got %d", mockDB.queryCalls)
	}
}

func TestSecretsSyncTool_Execute_ValidPush(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewSecretsSyncTool(proxy)

	keys := []string{"api_key", "db_password"}
	err := tool.Execute(ctx, keys, "push")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if mockDB.execCalls != 2 {
		t.Errorf("Expected 2 Exec calls, got %d", mockDB.execCalls)
	}
	if mockDB.queryCalls != 1 {
		t.Errorf("Expected 1 Query call, got %d", mockDB.queryCalls)
	}
}

func TestSecretsSyncTool_Execute_InvalidDirection(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewSecretsSyncTool(proxy)

	keys := []string{"api_key"}
	err := tool.Execute(ctx, keys, "invalid")
	if err == nil {
		t.Fatalf("Expected an error for invalid direction, got nil")
	}

	expectedErr := "invalid sync direction: must be 'pull' or 'push'"
	if err.Error() != expectedErr {
		t.Errorf("Expected error '%s', got '%s'", expectedErr, err.Error())
	}
}

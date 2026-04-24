package mcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestSecretsSyncTool_Execute(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewSecretsSyncTool(proxy)

	// Test invalid direction
	err := tool.Execute(ctx, []string{"key1"}, "invalid")
	if err == nil {
		t.Errorf("Expected error for invalid direction, got nil")
	}

	// Test valid push
	err = tool.Execute(ctx, []string{"key1"}, "push")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if mockDB.execCalls != 2 { // 1 buffer, 1 update status
		t.Errorf("Expected 2 Exec calls, got %d", mockDB.execCalls)
	}
	if mockDB.queryCalls != 1 {
		t.Errorf("Expected 1 Query call, got %d", mockDB.queryCalls)
	}
}

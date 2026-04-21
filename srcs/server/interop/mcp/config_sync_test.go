package mcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestConfigSyncTool_Execute(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewConfigSyncTool(proxy)

	config := map[string]interface{}{
		"setting1": "value1",
		"setting2": 42,
	}

	err := tool.Execute(ctx, config, "push")
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

func TestConfigSyncTool_Execute_Failure(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError) // Simulate server error
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewConfigSyncTool(proxy)

	config := map[string]interface{}{
		"setting1": "value1",
	}

	err := tool.Execute(ctx, config, "push")
	if err != nil {
		t.Fatalf("Expected no error (failed sync is still a success for buffer/execute), got %v", err)
	}

	if mockDB.execCalls != 2 { // 1 buffer, 1 update status (to StatusFailed)
		t.Errorf("Expected 2 Exec calls, got %d", mockDB.execCalls)
	}
}

func TestConfigSyncTool_GetHash(t *testing.T) {
	tool := NewConfigSyncTool(nil) // Proxy not needed for GetHash

	config1 := map[string]interface{}{"key": "value"}
	config2 := map[string]interface{}{"key": "value"}
	config3 := map[string]interface{}{"key": "different"}

	hash1, err := tool.GetHash(config1)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	hash2, err := tool.GetHash(config2)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	hash3, err := tool.GetHash(config3)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if hash1 != hash2 {
		t.Errorf("Expected identical configs to have same hash, got %s and %s", hash1, hash2)
	}

	if hash1 == hash3 {
		t.Errorf("Expected different configs to have different hashes, got %s", hash1)
	}
}

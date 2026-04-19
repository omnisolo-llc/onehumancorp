package mcp

import (
	"context"
	"testing"
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
)

func TestConfigSyncTool_Execute_MarshalError(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)

	// Create a map with an unmarshalable value (e.g. a channel)
	config := map[string]interface{}{
		"unmarshalable": make(chan int),
	}

	err := tool.Execute(ctx, config, "push")
	if err == nil {
		t.Fatalf("Expected error due to unmarshalable config, got nil")
	}
}

func TestConfigSyncTool_GetHash_MarshalError(t *testing.T) {
	tool := NewConfigSyncTool(nil)

	// Create a map with an unmarshalable value
	config := map[string]interface{}{
		"unmarshalable": make(chan int),
	}

	_, err := tool.GetHash(config)
	if err == nil {
		t.Fatalf("Expected error due to unmarshalable config, got nil")
	}
}

// mockDBProvider_FailExec is a mock DB provider that fails on Exec
type mockDBProvider_FailExec struct {
	mockDBProvider
}

func (m *mockDBProvider_FailExec) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, context.DeadlineExceeded
}

func TestConfigSyncTool_Execute_BufferError(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider_FailExec{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)

	config := map[string]interface{}{
		"setting1": "value1",
	}

	err := tool.Execute(ctx, config, "push")
	if err == nil {
		t.Fatalf("Expected error due to buffer failure, got nil")
	}
}

func TestConfigSyncTool_Execute_ValidationErrors(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)

	// Long key
	longKey := string(make([]byte, 257))
	config1 := map[string]interface{}{longKey: "val"}
	if err := tool.Execute(ctx, config1, "push"); err == nil {
		t.Fatalf("Expected validation error for long key, got nil")
	}

	// Long string val
	longVal := string(make([]byte, 10241))
	config2 := map[string]interface{}{"key": longVal}
	if err := tool.Execute(ctx, config2, "push"); err == nil {
		t.Fatalf("Expected validation error for long value, got nil")
	}
}

func TestConfigSyncTool_Execute_EncryptionHooks(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)

	config := map[string]interface{}{
		"local_proxy_password": "supersecret",
		"api_key":              "supersecret",
		"secret":               "supersecret",
		"normal":               "visible",
		"not_a_string":         42, // Non-string shouldn't be encrypted
	}

	// Ensure encryption functions work without panic
	encConfig := tool.encryptSensitive(config)
	if encConfig["local_proxy_password"] == "supersecret" {
		t.Fatalf("Expected 'local_proxy_password' to be encrypted")
	}
	if encConfig["normal"] != "visible" {
		t.Fatalf("Expected 'normal' to remain visible")
	}
	if encConfig["not_a_string"] != 42 {
		t.Fatalf("Expected 'not_a_string' to remain as is")
	}

	decConfig := tool.decryptSensitive(encConfig)
	if decConfig["local_proxy_password"] != "supersecret" {
		t.Fatalf("Expected 'local_proxy_password' to be decrypted")
	}
	if decConfig["normal"] != "visible" {
		t.Fatalf("Expected 'normal' to remain visible")
	}
}

func TestSyncAPIHandler_PutConfig_BadPayload(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)
	handler := NewSyncAPIHandler(tool)

	req := httptest.NewRequest(http.MethodPut, "/api/v1/sync/config", bytes.NewBuffer([]byte("{bad json}")))
	rr := httptest.NewRecorder()

	handler.handlePutConfig(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusBadRequest)
	}
}

func TestSyncAPIHandler_PutConfig_ExecuteError(t *testing.T) {
	mockDB := &mockDBProvider_FailExec{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)
	handler := NewSyncAPIHandler(tool)

	payload := ConfigPayload{
		ConfigData: map[string]interface{}{"key": "value"},
		Hash:       "dummyhash",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPut, "/api/v1/sync/config", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	handler.handlePutConfig(rr, req)

	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusInternalServerError)
	}
}

func TestConfigSyncTool_Execute_ValidationArrayHooks(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)

	// Array with long string val
	longVal := string(make([]byte, 10241))
	config2 := map[string]interface{}{"key": []interface{}{longVal}}
	if err := tool.Execute(ctx, config2, "push"); err == nil {
		t.Fatalf("Expected validation error for long value inside array, got nil")
	}
}

func TestConfigSyncTool_Execute_EncryptionArrayHooks(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)

	config := map[string]interface{}{
		"api_key": []interface{}{"supersecret1", "supersecret2"},
	}

	encConfig := tool.encryptSensitive(config)
	arr := encConfig["api_key"].([]interface{})
	if arr[0] == "supersecret1" {
		t.Fatalf("Expected array item 0 to be encrypted")
	}

	decConfig := tool.decryptSensitive(encConfig)
	decArr := decConfig["api_key"].([]interface{})
	if decArr[0] != "supersecret1" {
		t.Fatalf("Expected array item 0 to be decrypted")
	}
}

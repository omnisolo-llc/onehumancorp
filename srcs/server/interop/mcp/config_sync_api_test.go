package mcp

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestSyncAPIHandler_GetHash(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)
	handler := NewSyncAPIHandler(tool)

	req := httptest.NewRequest(http.MethodGet, "/api/v1/sync/config/hash", nil)
	rr := httptest.NewRecorder()

	handler.handleGetHash(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]string
	json.NewDecoder(rr.Body).Decode(&resp)

	if resp["hash"] == "" {
		t.Errorf("Expected valid hash, got empty string")
	}
}

func TestSyncAPIHandler_PutConfig(t *testing.T) {
	mockDB := &mockDBProvider{}
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

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestSyncAPIHandler_InvalidMethods(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)
	handler := NewSyncAPIHandler(tool)

	req1 := httptest.NewRequest(http.MethodPost, "/api/v1/sync/config/hash", nil)
	rr1 := httptest.NewRecorder()
	handler.handleGetHash(rr1, req1)
	if rr1.Code != http.StatusMethodNotAllowed {
		t.Errorf("Expected method not allowed, got %v", rr1.Code)
	}

	req2 := httptest.NewRequest(http.MethodPost, "/api/v1/sync/config", nil)
	rr2 := httptest.NewRecorder()
	handler.handlePutConfig(rr2, req2)
	if rr2.Code != http.StatusMethodNotAllowed {
		t.Errorf("Expected method not allowed, got %v", rr2.Code)
	}
}

func TestSyncAPIHandler_RegisterRoutes(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")
	tool := NewConfigSyncTool(proxy)
	handler := NewSyncAPIHandler(tool)

	mux := http.NewServeMux()
	handler.RegisterRoutes(mux)

	// Test GET route existence
	req1 := httptest.NewRequest(http.MethodGet, "/api/v1/sync/config/hash", nil)
	rr1 := httptest.NewRecorder()
	mux.ServeHTTP(rr1, req1)
	if rr1.Code == http.StatusNotFound {
		t.Errorf("Expected route /api/v1/sync/config/hash to be registered")
	}

	// Test PUT route existence
	req2 := httptest.NewRequest(http.MethodPut, "/api/v1/sync/config", bytes.NewBuffer([]byte(`{}`)))
	rr2 := httptest.NewRecorder()
	mux.ServeHTTP(rr2, req2)
	if rr2.Code == http.StatusNotFound {
		t.Errorf("Expected route /api/v1/sync/config to be registered")
	}
}

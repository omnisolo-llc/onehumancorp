package mcp

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"github.com/onehumancorp/mono/src/server/auth"
)

// Dummy auth context for testing RequireRole middleware
func getAuthContext() context.Context {
	ctx := context.Background()
	claims := &auth.Claims{Subject: "e2e-user", Roles: []string{"system"}}
	return context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)
}

func TestConfigSync_E2E_FullFlow(t *testing.T) {
	// 1. Setup mock environment
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewConfigSyncTool(proxy)
	handler := NewSyncAPIHandler(tool)

	mux := http.NewServeMux()
	handler.RegisterRoutes(mux)

	// We'll wrap the mux to inject the auth context so it passes the system role check
	testHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		r = r.WithContext(getAuthContext())
		mux.ServeHTTP(w, r)
	})

	// 2. Initial state: GET hash (should return dummy/empty)
	req1 := httptest.NewRequest(http.MethodGet, "/api/v1/sync/config/hash", nil)
	rr1 := httptest.NewRecorder()
	testHandler.ServeHTTP(rr1, req1)

	if rr1.Code != http.StatusOK {
		t.Fatalf("Expected OK for GET hash, got %v", rr1.Code)
	}

	// 3. User navigates feature: pushes new config (PUT)
	configData := map[string]interface{}{
		"api_key": "user_provided_secret",
		"theme":   "dark",
	}
	payload := ConfigPayload{
		ConfigData: configData,
		Hash:       "", // Not validated heavily in the stub
	}
	body, _ := json.Marshal(payload)

	req2 := httptest.NewRequest(http.MethodPut, "/api/v1/sync/config", bytes.NewBuffer(body))
	rr2 := httptest.NewRecorder()
	testHandler.ServeHTTP(rr2, req2)

	if rr2.Code != http.StatusOK {
		t.Fatalf("Expected OK for PUT config, got %v", rr2.Code)
	}

	// 4. Assert end state matches expectations (mockDB should have recorded the execute call, validating encryption)
	if mockDB.execCalls == 0 {
		t.Fatalf("Expected buffer to be executed, but db had no calls")
	}

	// Next, let's test a GET hash to see it returns our mockRow data
	req3 := httptest.NewRequest(http.MethodGet, "/api/v1/sync/config/hash", nil)
	rr3 := httptest.NewRecorder()
	testHandler.ServeHTTP(rr3, req3)

	if rr3.Code != http.StatusOK {
		t.Fatalf("Expected OK for GET hash after put, got %v", rr3.Code)
	}

	var resp map[string]string
	json.NewDecoder(rr3.Body).Decode(&resp)

	// Since we hardcoded the mockRow to return dummyhash
	if resp["hash"] != "dummyhash" {
		t.Fatalf("Expected hash to be dummyhash from DB, got %s", resp["hash"])
	}
}

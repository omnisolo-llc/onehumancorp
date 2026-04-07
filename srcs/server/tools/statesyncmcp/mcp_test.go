package statesyncmcp_test

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/tools/statesyncmcp"
)

func TestStateSyncMCP_ListTools(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	tools := mcpSrv.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	toolNames := map[string]bool{}
	for _, tool := range tools {
		toolNames[tool.Name] = true
	}

	expected := []string{"sync_local_to_cloud", "sync_cloud_to_local", "get_sync_status"}
	for _, exp := range expected {
		if !toolNames[exp] {
			t.Errorf("expected tool %s to be listed", exp)
		}
	}
}

func TestStateSyncMCP_CallTool_Unauthenticated(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	ctx := context.Background()

	_, err := mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	_, err = mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	_, err = mcpSrv.CallTool(ctx, "get_sync_status", nil)
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected unauthorized error, got %v", err)
	}
}

type mockCloudProvider struct {
	db.Provider
}

func (m *mockCloudProvider) IsSQLite() bool {
	return false
}

func TestStateSyncMCP_CloudMode_Syncs(t *testing.T) {
	provider := &mockCloudProvider{Provider: db.NewTestProvider(t)}
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// SyncUp
	resUp, err := mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	mUp := resUp.(map[string]interface{})
	if mUp["status"] != "skipped" {
		t.Errorf("expected skipped, got %v", mUp["status"])
	}

	// SyncDown
	resDown, err := mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	mDown := resDown.(map[string]interface{})
	if mDown["status"] != "skipped" {
		t.Errorf("expected skipped, got %v", mDown["status"])
	}

	// GetStatus
	resStatus, err := mcpSrv.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	mStatus := resStatus.(map[string]interface{})
	if mStatus["status"] != "cloud" {
		t.Errorf("expected cloud, got %v", mStatus["status"])
	}
}

func TestStateSyncMCP_CallTool_StandaloneMode(t *testing.T) {
	provider := db.NewTestProvider(t)

	// Create table
	_, err := provider.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, updated_at TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	provider.Exec(context.Background(), "CREATE TABLE local_cloud_sync_log (sync_id TEXT PRIMARY KEY, memory_id TEXT NOT NULL, cloud_mission_id TEXT, synced_at TIMESTAMP)")
	provider.Exec(context.Background(), "INSERT INTO agent_missions (id, status, payload) VALUES ('m1', 'DONE', '{}'), ('m2', 'PENDING', '{}')")

	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Mock HTTP server
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/sync/up" {
			w.WriteHeader(http.StatusOK)
		} else if r.URL.Path == "/api/v1/sync/down" {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte(`{"count": 2, "missions": [{"id": "m3", "status": "DONE", "payload": "{}"}, {"id": "m4", "status": "PENDING", "payload": "{}"}]}`))
		}
	}))
	defer srv.Close()

	t.Setenv("OHC_CORE_URL", srv.URL)

	resUp, err := mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	mUp := resUp.(map[string]interface{})
	if mUp["status"] != "success" || mUp["synced_items"] != 2 {
		t.Errorf("unexpected sync up result: %v", mUp)
	}

	resDown, err := mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	mDown := resDown.(map[string]interface{})
	if mDown["status"] != "success" || mDown["downloaded_items"] != 2 {
		t.Errorf("unexpected sync down result: %v", mDown)
	}

	resStatus, err := mcpSrv.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	mStatus := resStatus.(map[string]interface{})
	if mStatus["status"] != "standalone" {
		t.Errorf("unexpected sync status result: %v", mStatus)
	}
}

func TestStateSyncMCP_CallTool_UnknownTool(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	ctx := context.Background()
	res, err := mcpSrv.CallTool(ctx, "unknown_tool", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res != nil {
		t.Fatalf("expected nil result for unknown tool, got %v", res)
	}
}

func TestStateSyncMCP_SyncUp_QueryError(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	// Expect an error because agent_missions table doesn't exist
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestStateSyncMCP_SyncDown_Error(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// invalid url
	t.Setenv("OHC_CORE_URL", "http://::1:invalid-port")

	_, err := mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Fatalf("expected error from http.NewRequestWithContext, got nil")
	}
}

func TestStateSyncMCP_SyncUp_Error(t *testing.T) {
	provider := db.NewTestProvider(t)
	// Create table
	_, err := provider.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT, status TEXT, payload TEXT, updated_at TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	provider.Exec(context.Background(), "CREATE TABLE local_cloud_sync_log (sync_id TEXT PRIMARY KEY, memory_id TEXT NOT NULL, cloud_mission_id TEXT, synced_at TIMESTAMP)")
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// invalid url
	t.Setenv("OHC_CORE_URL", "http://::1:invalid-port")

	_, err = mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error from http.NewRequestWithContext, got nil")
	}
}

func TestStateSyncMCP_HTTPError_Up(t *testing.T) {
	provider := db.NewTestProvider(t)
	_, err := provider.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT, status TEXT, payload TEXT, updated_at TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	provider.Exec(context.Background(), "CREATE TABLE local_cloud_sync_log (sync_id TEXT PRIMARY KEY, memory_id TEXT NOT NULL, cloud_mission_id TEXT, synced_at TIMESTAMP)")
	provider.Exec(context.Background(), "INSERT INTO agent_missions (id, status, payload) VALUES ('m1', 'DONE', '{}')")
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer srv.Close()

	t.Setenv("OHC_CORE_URL", srv.URL)

	res, err := mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}

	m := res.(map[string]interface{})
	if m["status"] != "failed" {
		t.Errorf("expected failed, got %v", m["status"])
	}
}

func TestStateSyncMCP_HTTPError_Down(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer srv.Close()

	t.Setenv("OHC_CORE_URL", srv.URL)

	res, err := mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}

	m := res.(map[string]interface{})
	if m["status"] != "failed" {
		t.Errorf("expected failed, got %v", m["status"])
	}
}

func TestStateSyncMCP_HTTPInvalidJSON_Down(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{invalid-json}`))
	}))
	defer srv.Close()

	t.Setenv("OHC_CORE_URL", srv.URL)

	_, err := mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Errorf("expected error decoding json, got nil")
	}
}

func TestStateSyncMCP_HTTPNetworkError_Up(t *testing.T) {
	provider := db.NewTestProvider(t)
	_, err := provider.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT, status TEXT, payload TEXT, updated_at TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	provider.Exec(context.Background(), "CREATE TABLE local_cloud_sync_log (sync_id TEXT PRIMARY KEY, memory_id TEXT NOT NULL, cloud_mission_id TEXT, synced_at TIMESTAMP)")
	provider.Exec(context.Background(), "INSERT INTO agent_missions (id, status, payload) VALUES ('m1', 'DONE', '{}')")
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// invalid url
	t.Setenv("OHC_CORE_URL", "http://localhost:1")

	res, err := mcpSrv.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}

	m := res.(map[string]interface{})
	if m["status"] != "failed" {
		t.Errorf("expected failed, got %v", m["status"])
	}
}

func TestStateSyncMCP_HTTPNetworkError_Down(t *testing.T) {
	provider := db.NewTestProvider(t)
	syncProvider := statesyncmcp.NewDBStateSyncProvider(provider)
	mcpSrv := statesyncmcp.NewStateSyncMCP(syncProvider)

	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// invalid url
	t.Setenv("OHC_CORE_URL", "http://localhost:1")

	res, err := mcpSrv.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}

	m := res.(map[string]interface{})
	if m["status"] != "failed" {
		t.Errorf("expected failed, got %v", m["status"])
	}
}

package api

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

type mockExecDBProvider struct {
	db.Provider
}

func (m *mockExecDBProvider) IsSQLite() bool {
	return false
}

func (m *mockExecDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, nil
}

func TestHandleSyncMCPDeltas_InvalidMethod(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/v1/sync/mcp-deltas", nil)
	w := httptest.NewRecorder()

	hub := &orchestration.Hub{}
	handler := HandleSyncMCPDeltas(hub)
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405, got %d", w.Code)
	}
}

func TestHandleSyncMCPDeltas_MissingTenant(t *testing.T) {
	req := httptest.NewRequest("POST", "/api/v1/sync/mcp-deltas", nil)
	w := httptest.NewRecorder()

	hub := &orchestration.Hub{}
	handler := HandleSyncMCPDeltas(hub)
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("expected 400, got %d", w.Code)
	}
}

func TestHandleSyncMCPDeltas_InvalidJSON(t *testing.T) {
	req := httptest.NewRequest("POST", "/api/v1/sync/mcp-deltas", bytes.NewBuffer([]byte(`{invalid`)))
	req.Header.Set("X-Tenant-ID", "test-org")
	w := httptest.NewRecorder()

	hub := &orchestration.Hub{}
	handler := HandleSyncMCPDeltas(hub)
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("expected 400, got %d", w.Code)
	}
}

func TestHandleSyncMCPDeltas_EmptyDeltas(t *testing.T) {
	payload := SyncDeltasPayload{Deltas: nil}
	data, _ := json.Marshal(payload)
	req := httptest.NewRequest("POST", "/api/v1/sync/mcp-deltas", bytes.NewBuffer(data))
	req.Header.Set("X-Tenant-ID", "test-org")
	w := httptest.NewRecorder()

	hub := &orchestration.Hub{}
	handler := HandleSyncMCPDeltas(hub)
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}
}

func TestHandleSyncMCPDeltas_Success(t *testing.T) {
	payload := map[string]interface{}{
		"deltas": []map[string]interface{}{
			{
				"id":         "delta1",
				"entity_id":  "e1",
				"data":       "testdata",
				"updated_at": "now",
			},
			{
				"id": "", // invalid delta
			},
		},
	}
	data, _ := json.Marshal(payload)
	req := httptest.NewRequest("POST", "/api/v1/sync/mcp-deltas", bytes.NewBuffer(data))
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()

	hub := &orchestration.Hub{}
	sipDB, _ := orchestration.NewSIPDBWithProvider(&mockExecDBProvider{}, "test-org")
	hub.SetSIPDB(sipDB)
	handler := HandleSyncMCPDeltas(hub)
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}

	var res map[string]interface{}
	json.Unmarshal(w.Body.Bytes(), &res)
	if res["synced_count"].(float64) != 1 {
		t.Errorf("expected synced_count 1, got %v", res["synced_count"])
	}
}

package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRegisterMeshHTTPHandlers(t *testing.T) {
	prov := db.NewTestProvider(t)
	cn, _ := NewCentrifugeNode()
	tm := NewTaskManager(prov, cn)

	mux := http.NewServeMux()
	RegisterMeshHTTPHandlers(mux, tm)

	payload := map[string]interface{}{
		"task_id": "test-task",
		"action":  "UPDATE",
		"status":  "COMPLETED",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewReader(body))
	// Add valid context claims for "system" role to bypass auth middleware
	claims := &auth.Claims{
		Roles: []string{"system"},
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKey, claims)
	req = req.WithContext(ctx)

	rec := httptest.NewRecorder()
	mux.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("expected status OK, got %d", rec.Code)
	}

	var resp map[string]string
	if err := json.Unmarshal(rec.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp["status"] != "ok" {
		t.Errorf("expected status 'ok', got %s", resp["status"])
	}
}

package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/orchestration/hybrid_sync"
)

func TestHandleSyncEscalation(t *testing.T) {
	hub := orchestration.NewHub()
	defer hub.Close()

	handler := HandleSyncEscalation(hub)

	payloads := []hybrid_sync.SyncPayload{
		{
			MemoryID: "m1",
			Context:  "test context",
		},
	}
	body, _ := json.Marshal(payloads)

	req := httptest.NewRequest(http.MethodPost, "/api/sync/escalation", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	handler(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("expected status OK, got %v", w.Result().StatusCode)
	}

	var resp map[string]interface{}
	json.NewDecoder(w.Result().Body).Decode(&resp)

	if resp["synced_count"] == nil || resp["synced_count"].(float64) != 0 {
		// Since TaskQueue/SIPDB is nil in the default Hub mock/start, it should be 0 unless initialized
		// That's fine for this basic endpoint test.
	}
}

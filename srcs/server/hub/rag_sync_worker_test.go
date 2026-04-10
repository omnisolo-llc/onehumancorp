package hub

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestRAGSyncWorker(t *testing.T) {
	svc := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}

	serverCalled := false
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		serverCalled = true
		if r.URL.Path != "/api/v1/sync/rag" {
			t.Errorf("expected path /api/v1/sync/rag, got %s", r.URL.Path)
		}
		if r.Method != "POST" {
			t.Errorf("expected method POST, got %s", r.Method)
		}
		auth := r.Header.Get("Authorization")
		if auth != "Bearer test-key" {
			t.Errorf("expected Bearer test-key, got %s", auth)
		}
		var reqs []RAGSyncRecord
		json.NewDecoder(r.Body).Decode(&reqs)
		if len(reqs) != 1 || reqs[0].ID != "1" {
			t.Errorf("unexpected payload: %+v", reqs)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	worker := NewRAGSyncWorker(svc, ts.URL, "test-key")
	worker.syncPendingRecords(context.Background())

	if !serverCalled {
		t.Errorf("server was not called")
	}

	if svc.Records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("record was not marked as synced")
	}
}

func TestRAGSyncHandler(t *testing.T) {
	svc := &MockRAGSyncService{}
	handler := NewRAGSyncHandler(svc)

	reqPayload := `[{"ID": "test-id", "Context": "test-context"}]`
	req := httptest.NewRequest("POST", "/api/v1/sync/rag", strings.NewReader(reqPayload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	handler.ServeHTTP(w, req)

	resp := w.Result()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("expected 200 OK, got %d", resp.StatusCode)
	}

	if len(svc.Records) != 1 || svc.Records[0].ID != "test-id" {
		t.Errorf("record was not processed: %+v", svc.Records)
	}
}

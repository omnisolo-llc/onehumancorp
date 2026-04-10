package hub

import (
	"context"
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandleRAGSync_Coverage(t *testing.T) {
	// Setup mock service and hub
	mockService := &MockRAGSyncService{}
	hub := &Hub{
		RAGSyncService: mockService,
	}

	// Prepare payload
	records := []RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "test context",
			Vector:     []byte("test vector"),
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	body, _ := json.Marshal(records)

	// Test 1: Unauthorized
	req := httptest.NewRequest(http.MethodPost, "/api/sync/rag", bytes.NewReader(body))
	w := httptest.NewRecorder()
	hub.HandleRAGSync(w, req)
	if w.Code != http.StatusUnauthorized {
		t.Errorf("expected 401 Unauthorized, got %d", w.Code)
	}

	// Test 2: Success
	req = httptest.NewRequest(http.MethodPost, "/api/sync/rag", bytes.NewReader(body))
	// Add mock claims to context
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org1",
		Subject:        "user1",
	})
	req = req.WithContext(ctx)

	w = httptest.NewRecorder()
	hub.HandleRAGSync(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected 200 OK, got %d", w.Code)
	}
	if len(mockService.IncomingRecords) != 1 {
		t.Errorf("expected 1 record processed, got %d", len(mockService.IncomingRecords))
	}

	// Test 3: Bad Request
	req = httptest.NewRequest(http.MethodPost, "/api/sync/rag", bytes.NewReader([]byte("invalid json")))
	req = req.WithContext(ctx)
	w = httptest.NewRecorder()
	hub.HandleRAGSync(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request, got %d", w.Code)
	}
}

package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

type mockCrdtStore struct {
	inserted int
}

func (m *mockCrdtStore) InsertOrUpdateDelta(ctx context.Context, id, entityID, data string, updatedAt time.Time) error {
	m.inserted++
	return nil
}

func TestCrdtSyncHandler(t *testing.T) {
	mockStore := &mockCrdtStore{}
	handler := CrdtSyncHandler(mockStore)

	payload := CrdtSyncPayload{
		Deltas: []CrdtDelta{
			{ID: "1", EntityID: "a", Data: "{}", UpdatedAt: time.Now()},
			{ID: "2", EntityID: "b", Data: "{}", UpdatedAt: time.Now()},
		},
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest("POST", "/sync/mcp-deltas", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")

	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", rr.Code)
	}

	if mockStore.inserted != 2 {
		t.Errorf("expected 2 inserts, got %d", mockStore.inserted)
	}
}

func TestCrdtSyncHandler_InvalidMethod(t *testing.T) {
	mockStore := &mockCrdtStore{}
	handler := CrdtSyncHandler(mockStore)
	req := httptest.NewRequest("GET", "/sync/mcp-deltas", nil)
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected status 405, got %d", rr.Code)
	}
}

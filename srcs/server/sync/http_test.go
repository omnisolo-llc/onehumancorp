package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

type MockService struct{}

func (m *MockService) SyncDeltas(ctx context.Context, deltas []SyncDelta) error {
	return nil
}

func TestHTTPHandler(t *testing.T) {
	handler := NewHTTPHandler(&MockService{})

	t.Run("Unauthorized_MissingContext", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/v1/sync/mcp-deltas", nil)
		w := httptest.NewRecorder()
		handler.HandleSync(w, req)

		if w.Code != http.StatusUnauthorized {
			t.Errorf("Expected 401, got %d", w.Code)
		}
	})

	t.Run("Success", func(t *testing.T) {
		deltas := []SyncDelta{{TenantID: "t1"}}
		body, _ := json.Marshal(deltas)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/sync/mcp-deltas", bytes.NewBuffer(body))

		// Add tenant to context
		ctx := context.WithValue(req.Context(), "tenant_id", "t1")
		req = req.WithContext(ctx)

		w := httptest.NewRecorder()
		handler.HandleSync(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("Expected 200, got %d", w.Code)
		}
	})
}

package api

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestPowerSyncHandler(t *testing.T) {
	h := NewPowerSyncHandler(nil)

	t.Run("Push Valid", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/v1/sync/push", bytes.NewBuffer([]byte(`[{"id": "1", "name": "test"}]`)))
		w := httptest.NewRecorder()

		h.ServeHTTP(w, req)

		if w.Result().StatusCode != http.StatusOK {
			t.Errorf("Expected 200 OK, got %d", w.Result().StatusCode)
		}
	})

	t.Run("Pull", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/v1/sync/pull", nil)
		w := httptest.NewRecorder()

		h.ServeHTTP(w, req)

		if w.Result().StatusCode != http.StatusOK {
			t.Errorf("Expected 200 OK, got %d", w.Result().StatusCode)
		}
	})

	t.Run("Invalid Path", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/api/v1/sync/invalid", nil)
		w := httptest.NewRecorder()

		h.ServeHTTP(w, req)

		if w.Result().StatusCode != http.StatusNotFound {
			t.Errorf("Expected 404 Not Found, got %d", w.Result().StatusCode)
		}
	})
}

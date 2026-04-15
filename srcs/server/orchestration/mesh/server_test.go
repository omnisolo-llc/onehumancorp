package mesh

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleBroadcast(t *testing.T) {
	body := []byte(`{"agent_id": "1", "action": "test", "status": "ok"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(body))
	w := httptest.NewRecorder()
	HandleBroadcast(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}
}

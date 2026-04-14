package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

type mockBroker struct {
	broadcasted bool
}

func (m *mockBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	m.broadcasted = true
	return nil
}

func TestHTTPHandler(t *testing.T) {
	broker := &mockBroker{}
	handler := NewHTTPHandler(broker)

	reqBody := []byte(`{"channel":"test","event_type":"msg","data":{"key":"value"}}`)
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewReader(reqBody))
	rr := httptest.NewRecorder()

	handler.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected status OK, got %v", rr.Code)
	}
	if !broker.broadcasted {
		t.Errorf("expected broadcast to be called")
	}
}

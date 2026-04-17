package orchestration

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

type mockMeshTransport struct {
	MeshTransport
	broadcastCalled bool
	subChan         chan []byte
}

func (m *mockMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	m.broadcastCalled = true
	return nil
}

func (m *mockMeshTransport) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	return m.subChan, nil
}

func TestMeshAPI_Broadcast(t *testing.T) {
	mockMesh := &mockMeshTransport{}
	api := NewMeshAPI(mockMesh)

	// Test valid payload
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{"agent_id": "spiffe://onehumancorp.io/agent/test", "channel": "mesh:tasks", "event_type": "TEST", "data": {}}`)))
	w := httptest.NewRecorder()

	api.HandleBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	if !mockMesh.broadcastCalled {
		t.Errorf("expected BroadcastMeshEvent to be called")
	}

	// Test missing/invalid agent_id
	req = httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{"agent_id": "invalid", "channel": "mesh:tasks", "event_type": "TEST", "data": {}}`)))
	w = httptest.NewRecorder()
	api.HandleBroadcast(w, req)
	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status 400 for invalid agent_id, got %d", w.Code)
	}

	// Test invalid channel
	req = httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{"agent_id": "spiffe://onehumancorp.io/agent/test", "channel": "invalid_channel", "event_type": "TEST", "data": {}}`)))
	w = httptest.NewRecorder()
	api.HandleBroadcast(w, req)
	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status 400 for invalid channel, got %d", w.Code)
	}
}

func TestMeshAPI_Stream(t *testing.T) {
	mockMesh := &mockMeshTransport{
		subChan: make(chan []byte, 1),
	}
	mockMesh.subChan <- []byte(`{"status":"test"}`)

	api := NewMeshAPI(mockMesh)

	req := httptest.NewRequest(http.MethodGet, "/api/mesh/stream", nil)
	w := httptest.NewRecorder()

	// Use a context with timeout to stop the infinite loop in HandleStream
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	req = req.WithContext(ctx)

	api.HandleStream(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	body := w.Body.String()
	if body != "data: {\"status\":\"test\"}\n\n" {
		t.Errorf("expected correct SSE format, got %s", body)
	}
}

func TestMeshAPI_Publish(t *testing.T) {
	mockMesh := &mockMeshTransport{}
	api := NewMeshAPI(mockMesh)

	// Test valid payload
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/publish", bytes.NewBuffer([]byte(`{"agent_id": "spiffe://onehumancorp.io/agent/test", "channel": "mesh:tasks", "event_type": "TEST", "data": {}}`)))
	w := httptest.NewRecorder()
	api.HandlePublish(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	// Test missing/invalid agent_id
	req = httptest.NewRequest(http.MethodPost, "/api/mesh/publish", bytes.NewBuffer([]byte(`{"agent_id": "invalid", "channel": "mesh:tasks", "event_type": "TEST", "data": {}}`)))
	w = httptest.NewRecorder()
	api.HandlePublish(w, req)
	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status 400 for invalid agent_id, got %d", w.Code)
	}

	// Test invalid channel
	req = httptest.NewRequest(http.MethodPost, "/api/mesh/publish", bytes.NewBuffer([]byte(`{"agent_id": "spiffe://onehumancorp.io/agent/test", "channel": "invalid_channel", "event_type": "TEST", "data": {}}`)))
	w = httptest.NewRecorder()
	api.HandlePublish(w, req)
	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status 400 for invalid channel, got %d", w.Code)
	}
}

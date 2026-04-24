package orchestration

import (
	"github.com/onehumancorp/mono/src/server/auth"

	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

type mockMeshApiTransport struct {
	MeshTransport
	broadcastCalled bool
	subChan         chan []byte
}

func (m *mockMeshApiTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	m.broadcastCalled = true
	return nil
}

func (m *mockMeshApiTransport) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	return m.subChan, nil
}

func TestMeshAPI_Broadcast(t *testing.T) {
	mockMesh := &mockMeshApiTransport{}
	api := NewMeshAPI(mockMesh)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{"task_id":"123"}`)))
	w := httptest.NewRecorder()
	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"}))
	api.HandleBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}
	if !mockMesh.broadcastCalled {
		t.Errorf("expected BroadcastMeshEvent to be called")
	}

	mockMesh.broadcastCalled = false
	req = httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{"channel":"ohc.mesh.agent.123", "task_id":"456"}`)))
	w = httptest.NewRecorder()
	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"}))
	api.HandleBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}
	if !mockMesh.broadcastCalled {
		t.Errorf("expected BroadcastMeshEvent to be called")
	}
}

func TestMeshAPI_Stream(t *testing.T) {
	mockMesh := &mockMeshApiTransport{
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

	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"}))
	api.HandleStream(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	body := w.Body.String()
	if body != "retry: 3000\n\ndata: {\"status\":\"test\"}\n\n" {
		t.Errorf("expected correct SSE format, got %s", body)
	}
}

func TestMeshAPI_HandleMeshV1Broadcast(t *testing.T) {
	mockMesh := &mockMeshApiTransport{}
	api := NewMeshAPI(mockMesh)

	tests := []struct {
		name       string
		method     string
		body       string
		statusCode int
	}{
		{"Method Not Allowed", http.MethodGet, "", http.StatusMethodNotAllowed},
		{"Invalid JSON", http.MethodPost, "{invalid}", http.StatusBadRequest},
		{"Missing Fields", http.MethodPost, `{"agent_id":"worker-1"}`, http.StatusBadRequest},
		{"Success", http.MethodPost, `{"agent_id":"worker-1","channel":"orchestration.tasks","action":"TaskTransition","status":"success"}`, http.StatusOK},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(tt.method, "/api/v1/mesh/broadcast", bytes.NewBuffer([]byte(tt.body)))
			w := httptest.NewRecorder()
			req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"}))
			api.HandleMeshV1Broadcast(w, req)
			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
		})
	}
}

func TestMeshAPI_Sync(t *testing.T) {
	mockMesh := &mockMeshApiTransport{
		subChan: make(chan []byte, 1),
	}
	mockMesh.subChan <- []byte(`{"sync_status":"ok"}`)

	api := NewMeshAPI(mockMesh)

	req := httptest.NewRequest(http.MethodGet, "/api/mesh/sync?channel=ohc.mesh.agent.123", nil)
	w := httptest.NewRecorder()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	req = req.WithContext(ctx)

	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"}))
	api.HandleSync(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	body := w.Body.String()
	if body != "retry: 3000\n\ndata: {\"sync_status\":\"ok\"}\n\n" {
		t.Errorf("expected correct SSE format, got %s", body)
	}
}

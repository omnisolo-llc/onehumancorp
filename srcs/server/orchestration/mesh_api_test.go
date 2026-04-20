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

func (m *mockMeshTransport) SendDirectMessage(ctx context.Context, toAgentID string, msg MeshMessage) error {
	m.broadcastCalled = true
	return nil
}

func (m *mockMeshTransport) SubscribeDirectMessages(ctx context.Context, agentID string) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage, 1)
	ch <- MeshMessage{AgentID: agentID, Content: "test mailbox"}
	return ch, nil
}

func TestMeshAPI_Broadcast(t *testing.T) {
	mockMesh := &mockMeshTransport{}
	api := NewMeshAPI(mockMesh)

	payload := `{"agent_id":"agent-1","action":"mesh:tasks","status":"active","payload":{"task_id":"123"}}`
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(payload)))
	w := httptest.NewRecorder()

	api.HandleBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d: %s", w.Code, w.Body.String())
	}

	if !mockMesh.broadcastCalled {
		t.Errorf("expected BroadcastMeshEvent to be called")
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


func TestMeshAPI_HandleMeshV1Broadcast(t *testing.T) {
	mockMesh := &mockMeshTransport{}
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
			api.HandleMeshV1Broadcast(w, req)
			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
		})
	}
}

func TestMeshAPI_Direct(t *testing.T) {
	mockMesh := &mockMeshTransport{}
	api := NewMeshAPI(mockMesh)

	payload := `{"agent_id":"agent-1","action":"DIRECT","content":"hello"}`
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/direct", bytes.NewBuffer([]byte(payload)))
	w := httptest.NewRecorder()

	api.HandleDirect(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}
}

func TestMeshAPI_Mailbox(t *testing.T) {
	mockMesh := &mockMeshTransport{}
	api := NewMeshAPI(mockMesh)

	req := httptest.NewRequest(http.MethodGet, "/api/mesh/mailbox?agent_id=agent-1", nil)
	w := httptest.NewRecorder()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	req = req.WithContext(ctx)

	api.HandleMailbox(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}
	if !bytes.Contains(w.Body.Bytes(), []byte("test mailbox")) {
		t.Errorf("expected body to contain 'test mailbox', got %s", w.Body.String())
	}
}

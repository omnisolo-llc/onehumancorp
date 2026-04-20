package orchestration

import (
	"bytes"
	"context"
	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
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

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{"task_id":"123"}`)))
	w := httptest.NewRecorder()

	api.HandleBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
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

func TestMeshAPI_HandleBridgeConnect(t *testing.T) {
	// Mock a WebSocket server representing the remote swarm
	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()
	}))
	defer server.Close()
	wsURL := "ws" + server.URL[4:]

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{
		meshTransport: mt,
	}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	api := NewMeshAPI(&mockMeshTransport{})
	api.SetBridgeManager(bm)

	tests := []struct {
		name       string
		method     string
		body       string
		statusCode int
	}{
		{"Method Not Allowed", http.MethodGet, "", http.StatusMethodNotAllowed},
		{"Invalid JSON", http.MethodPost, "{invalid}", http.StatusBadRequest},
		{"Missing Fields", http.MethodPost, `{"remote_swarm_url":"` + wsURL + `"}`, http.StatusBadRequest},
		{"Success", http.MethodPost, `{"remote_swarm_url":"` + wsURL + `","remote_organization_id":"remote-org-1"}`, http.StatusOK},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(tt.method, "/api/v1/mesh/bridge/connect", bytes.NewBuffer([]byte(tt.body)))
			w := httptest.NewRecorder()
			api.HandleBridgeConnect(w, req)
			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
		})
	}
}

func TestMeshAPI_HandleBridgeStatus(t *testing.T) {
	api := NewMeshAPI(&mockMeshTransport{})

	// Test without BridgeManager
	req := httptest.NewRequest(http.MethodGet, "/api/v1/mesh/bridge/status", nil)
	w := httptest.NewRecorder()
	api.HandleBridgeStatus(w, req)
	if w.Code != http.StatusInternalServerError {
		t.Errorf("expected %d, got %d", http.StatusInternalServerError, w.Code)
	}

	// Test with BridgeManager
	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{
		meshTransport: mt,
	}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
	api.SetBridgeManager(bm)

	req = httptest.NewRequest(http.MethodGet, "/api/v1/mesh/bridge/status", nil)
	w = httptest.NewRecorder()
	api.HandleBridgeStatus(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected %d, got %d", http.StatusOK, w.Code)
	}

	if string(w.Body.Bytes()) != "{}" {
		t.Errorf("expected empty JSON object, got %s", string(w.Body.Bytes()))
	}
}

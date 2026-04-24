package orchestration

import (
	"bytes"
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type mockMeshTransport struct {
	MeshTransport
	subChan         chan []byte
	broadcastErr    error
	subscribeErr    error
	broadcastCalled bool
}

func (m *mockMeshTransport) BroadcastMeshEvent(ctx context.Context, channel string, payload []byte) error {
	m.broadcastCalled = true
	return m.broadcastErr
}

func (m *mockMeshTransport) SubscribeMeshEvents(ctx context.Context, channel string) (<-chan []byte, error) {
	if m.subscribeErr != nil {
		return nil, m.subscribeErr
	}
	return m.subChan, nil
}

func TestMeshAPI_Broadcast(t *testing.T) {
	mockMesh := &mockMeshTransport{}
	api := NewMeshAPI(mockMesh)

	tests := []struct {
		name       string
		method     string
		body       string
		statusCode int
	}{
		{
			name:       "valid request",
			method:     http.MethodPost,
			body:       `{"channel": "ohc.mesh.agent.123", "type": "event"}`,
			statusCode: http.StatusOK,
		},
		{
			name:       "missing channel defaults to tasks",
			method:     http.MethodPost,
			body:       `{"type": "event"}`,
			statusCode: http.StatusOK,
		},
		{
			name:       "invalid method",
			method:     http.MethodGet,
			body:       ``,
			statusCode: http.StatusMethodNotAllowed,
		},
		{
			name:       "invalid body json",
			method:     http.MethodPost,
			body:       `{invalid json}`,
			statusCode: http.StatusBadRequest,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(tt.method, "/api/mesh/broadcast", bytes.NewBuffer([]byte(tt.body)))
			w := httptest.NewRecorder()

			api.HandleBroadcast(w, req)

			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
			if tt.statusCode == http.StatusOK && !mockMesh.broadcastCalled {
				t.Errorf("expected BroadcastMeshEvent to be called for valid payload")
			}
			mockMesh.broadcastCalled = false
		})
	}
}

func TestMeshAPI_Stream(t *testing.T) {
	mockMesh := &mockMeshTransport{
		subChan: make(chan []byte, 1),
	}
	mockMesh.subChan <- []byte(`{"status":"ok"}`)

	api := NewMeshAPI(mockMesh)

	req := httptest.NewRequest(http.MethodGet, "/api/mesh/stream", nil)
	w := httptest.NewRecorder()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	req = req.WithContext(ctx)

	api.HandleStream(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	body := w.Body.String()
	if body != "retry: 3000\n\ndata: {\"status\":\"ok\"}\n\n" {
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
		{
			name:       "valid v1 broadcast",
			method:     http.MethodPost,
			body:       `{"agent_id": "agent-1", "channel": "mesh:tasks", "action": "update", "status": "done", "payload": {"foo": "bar"}}`,
			statusCode: http.StatusOK,
		},
		{
			name:       "missing required field",
			method:     http.MethodPost,
			body:       `{"agent_id": "agent-1", "action": "update", "status": "done"}`,
			statusCode: http.StatusBadRequest,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(tt.method, "/api/v1/mesh/broadcast", bytes.NewBuffer([]byte(tt.body)))
			w := httptest.NewRecorder()
			api.HandleMeshV1Broadcast(w, req)
			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
			if tt.statusCode == http.StatusOK && !mockMesh.broadcastCalled {
				t.Errorf("expected BroadcastMeshEvent to be called for valid payload")
			}
			mockMesh.broadcastCalled = false
		})
	}
}

func TestMeshAPI_Sync(t *testing.T) {
	mockMesh := &mockMeshTransport{
		subChan: make(chan []byte, 1),
	}
	mockMesh.subChan <- []byte(`{"sync_status":"ok"}`)

	api := NewMeshAPI(mockMesh)

	req := httptest.NewRequest(http.MethodGet, "/api/mesh/sync?channel=ohc.mesh.agent.123", nil)
	w := httptest.NewRecorder()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	req = req.WithContext(ctx)

	api.HandleSync(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	body := w.Body.String()
	if body != "retry: 3000\n\ndata: {\"sync_status\":\"ok\"}\n\n" {
		t.Errorf("expected correct SSE format, got %s", body)
	}
}

func TestMeshAPI_HandleMeshV2Broadcast(t *testing.T) {
	mockTransport := &mockMeshTransport{}
	api := NewMeshAPI(mockTransport)

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/mesh/v2/broadcast", nil)
		w := httptest.NewRecorder()
		api.HandleMeshV2Broadcast(w, req)

		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected status %d, got %d", http.StatusMethodNotAllowed, w.Code)
		}
	})

	t.Run("invalid payload", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewBuffer([]byte(`{`)))
		w := httptest.NewRecorder()
		api.HandleMeshV2Broadcast(w, req)

		if w.Code != http.StatusBadRequest {
			t.Errorf("expected status %d, got %d", http.StatusBadRequest, w.Code)
		}
	})

	t.Run("missing channel", func(t *testing.T) {
		event := &pb.PublishTeammateMeshEventRequest{Channel: ""}
		data, _ := proto.Marshal(event)
		req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewBuffer(data))
		w := httptest.NewRecorder()
		api.HandleMeshV2Broadcast(w, req)

		if w.Code != http.StatusBadRequest {
			t.Errorf("expected status %d, got %d", http.StatusBadRequest, w.Code)
		}
	})

	t.Run("valid payload", func(t *testing.T) {
		event := &pb.PublishTeammateMeshEventRequest{Channel: "mesh:tasks"}
		data, _ := proto.Marshal(event)
		req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewBuffer(data))
		w := httptest.NewRecorder()
		api.HandleMeshV2Broadcast(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
		}
		if !mockTransport.broadcastCalled {
			t.Errorf("expected BroadcastMeshEvent to be called")
		}
		mockTransport.broadcastCalled = false
	})

	t.Run("broadcast error", func(t *testing.T) {
		mockTransport.broadcastErr = errors.New("broadcast failed")
		event := &pb.PublishTeammateMeshEventRequest{Channel: "mesh:tasks"}
		data, _ := proto.Marshal(event)
		req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewBuffer(data))
		w := httptest.NewRecorder()
		api.HandleMeshV2Broadcast(w, req)

		if w.Code != http.StatusInternalServerError {
			t.Errorf("expected status %d, got %d", http.StatusInternalServerError, w.Code)
		}
		mockTransport.broadcastErr = nil
	})
}

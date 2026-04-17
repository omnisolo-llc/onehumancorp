package orchestration

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"os"
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


func TestMeshCoordinatorService_Local(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	m := NewMeshCoordinatorService(nil)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch, err := m.Subscribe(ctx, "test-channel")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	msg := MeshMessageDTO{
		ID:      "1",
		Sender:  "agent1",
		Channel: "test-channel",
		Content: "{}",
	}
	err = m.Publish(ctx, msg)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case received := <-ch:
		if received.ID != "1" {
			t.Fatalf("expected ID 1, got %v", received.ID)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

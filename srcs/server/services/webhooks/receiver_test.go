package webhooks

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// MockTeammateMeshService implements mesh.TeammateMeshService for testing
type MockTeammateMeshService struct {
	broadcasts chan string
}

func NewMockTeammateMeshService() *MockTeammateMeshService {
	return &MockTeammateMeshService{
		broadcasts: make(chan string, 100),
	}
}

func (m *MockTeammateMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	m.broadcasts <- intent
	return nil
}

func (m *MockTeammateMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	return m.broadcasts, nil
}

func TestWebhookReceiver_HandleIncoming(t *testing.T) {
	mockMesh := NewMockTeammateMeshService()
	receiver := NewWebhookReceiver(mockMesh)

	ctx := context.Background()
	reqBody := []byte(`{"event":"test"}`)
	req := httptest.NewRequestWithContext(ctx, http.MethodPost, "/webhook", bytes.NewReader(reqBody))
	rw := httptest.NewRecorder()

	sub, err := mockMesh.Subscribe(ctx)
	if err != nil {
		t.Fatalf("Failed to subscribe to mesh: %v", err)
	}

	receiver.HandleIncoming(rw, req)

	if rw.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %d", rw.Code)
	}

	// Verify message in mesh
	select {
	case msg := <-sub:
		var meshMsg orchestration.MeshMessage
		if err := json.Unmarshal([]byte(msg), &meshMsg); err != nil {
			t.Fatalf("Failed to unmarshal mesh message: %v", err)
		}
		if meshMsg.Action != "WebhookReceived" {
			t.Errorf("Expected action WebhookReceived, got %s", meshMsg.Action)
		}
		if meshMsg.Content != string(reqBody) {
			t.Errorf("Expected content %s, got %s", string(reqBody), meshMsg.Content)
		}
	case <-time.After(1 * time.Second):
		t.Errorf("Timeout waiting for mesh message")
	}
}

func TestWebhookReceiver_HandleIncoming_WrongMethod(t *testing.T) {
	mockMesh := NewMockTeammateMeshService()
	receiver := NewWebhookReceiver(mockMesh)

	req := httptest.NewRequest(http.MethodGet, "/webhook", nil)
	rw := httptest.NewRecorder()

	receiver.HandleIncoming(rw, req)

	if rw.Code != http.StatusMethodNotAllowed {
		t.Errorf("Expected status 405, got %d", rw.Code)
	}
}

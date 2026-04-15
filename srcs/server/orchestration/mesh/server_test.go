package mesh

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
    "context"
    "time"
)

type mockMesh struct{}

func (m *mockMesh) Publish(ctx context.Context, topic string, payload []byte) error { return nil }
func (m *mockMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) { return nil, nil }
func (m *mockMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) { return true, nil }
func (m *mockMesh) ReleaseLock(ctx context.Context, key string) error { return nil }
func (m *mockMesh) RegisterPresence(ctx context.Context, agentID string, status string) error { return nil }
func (m *mockMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) { return nil, nil }

func TestBroadcastHandler(t *testing.T) {
	mesh := &mockMesh{}
	handler := BroadcastHandler(mesh)

	req := httptest.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBufferString(`{"agent_id": "test", "action": "test", "status": "test"}`))
	rec := httptest.NewRecorder()

	handler.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", rec.Code)
	}
}

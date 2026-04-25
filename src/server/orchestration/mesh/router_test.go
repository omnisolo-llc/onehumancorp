package mesh

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type MockMesh struct {
	PublishedMessages [][]byte
}

func (m *MockMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.PublishedMessages = append(m.PublishedMessages, payload)
	return nil
}

func (m *MockMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	return nil, nil
}

func (m *MockMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (string, bool, error) {
	return "", true, nil
}

func (m *MockMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	return nil
}

func (m *MockMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return nil
}

func (m *MockMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	return nil, nil
}

func TestCapabilityRouter_EnsureTables(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	mockMesh := &MockMesh{}
	router := NewCapabilityRouter(mockMesh, provider)

	if err := router.EnsureTables(ctx); err != nil {
		t.Fatalf("failed to ensure tables: %v", err)
	}
}

func TestCapabilityRouter_RegisterAndDispatch(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	mockMesh := &MockMesh{}
	router := NewCapabilityRouter(mockMesh, provider)

	if err := router.EnsureTables(ctx); err != nil {
		t.Fatalf("failed to ensure tables: %v", err)
	}

	tenantID := "tenant-1"

	err := router.RegisterAgentProfile(ctx, tenantID, AgentProfile{
		AgentID: "agent-a",
		Skills:  []string{"marketing", "copywriting"},
		Status:  "AVAILABLE",
	})
	if err != nil {
		t.Fatalf("failed to register profile: %v", err)
	}

	err = router.RegisterAgentProfile(ctx, tenantID, AgentProfile{
		AgentID: "agent-b",
		Skills:  []string{"finance", "billing"},
		Status:  "BUSY",
	})
	if err != nil {
		t.Fatalf("failed to register profile: %v", err)
	}

	// Wait, agent-b is busy, agent-a is available but has different skills.
	// We want to dispatch a marketing job
	err = router.DispatchJob(ctx, tenantID, "job-1", "marketing", []byte(`{"campaign":"summer"}`))
	if err != nil {
		t.Fatalf("failed to dispatch job: %v", err)
	}

	if len(mockMesh.PublishedMessages) != 1 {
		t.Fatalf("expected 1 message published, got %d", len(mockMesh.PublishedMessages))
	}

	var msg map[string]interface{}
	if err := json.Unmarshal(mockMesh.PublishedMessages[0], &msg); err != nil {
		t.Fatalf("failed to unmarshal published msg: %v", err)
	}

	if msg["agent_id"] != "agent-a" {
		t.Fatalf("expected agent-a, got %v", msg["agent_id"])
	}

	if msg["tenant_id"] != "tenant-1" {
		t.Fatalf("expected tenant-1, got %v", msg["tenant_id"])
	}

	// Dispatch job that has no available agent
	err = router.DispatchJob(ctx, tenantID, "job-2", "finance", []byte(`{}`))
	if err == nil {
		t.Fatal("expected error for missing available skill, got nil")
	}

	err = router.DispatchJob(ctx, tenantID, "job-3", "programming", []byte(`{}`))
	if err == nil {
		t.Fatal("expected error for missing skill altogether, got nil")
	}

	err = router.DispatchJob(ctx, "", "job-4", "marketing", []byte(`{}`))
	if err == nil {
		t.Fatal("expected error for missing tenant_id, got nil")
	}

	// Update agent-a to be BUSY
	err = router.RegisterAgentProfile(ctx, tenantID, AgentProfile{
		AgentID: "agent-a",
		Skills:  []string{"marketing", "copywriting"},
		Status:  "BUSY",
	})
	if err != nil {
		t.Fatalf("failed to update profile: %v", err)
	}

	err = router.DispatchJob(ctx, tenantID, "job-5", "marketing", []byte(`{"campaign":"summer"}`))
	if err == nil {
		t.Fatal("expected error for no available agents with skill, got nil")
	}
}

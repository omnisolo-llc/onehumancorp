package mesh

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

// MockTeammateMesh for testing
type MockTeammateMeshRouter struct {
	mock.Mock
}

func (m *MockTeammateMeshRouter) Publish(ctx context.Context, topic string, payload []byte) error {
	args := m.Called(ctx, topic, payload)
	return args.Error(0)
}

func (m *MockTeammateMeshRouter) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	args := m.Called(ctx, topic, handler)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).(Subscription), args.Error(1)
}

func (m *MockTeammateMeshRouter) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	args := m.Called(ctx, key, ttl)
	return args.Bool(0), args.Error(1)
}

func (m *MockTeammateMeshRouter) ReleaseLock(ctx context.Context, key string) error {
	args := m.Called(ctx, key)
	return args.Error(0)
}

func (m *MockTeammateMeshRouter) RegisterPresence(ctx context.Context, agentID string, status string) error {
	args := m.Called(ctx, agentID, status)
	return args.Error(0)
}

func (m *MockTeammateMeshRouter) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	args := m.Called(ctx)
	if args.Get(0) == nil {
		return nil, args.Error(1)
	}
	return args.Get(0).([]AgentPresence), args.Error(1)
}

func TestCapabilityRouter(t *testing.T) {
	// Mock DB
	mockDB := db.NewTestProvider(t)

	// Create table and insert data
	_, err := mockDB.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			capabilities JSONB DEFAULT '[]',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	caps1, _ := json.Marshal([]string{"frontend", "react"})
	_, err = mockDB.Exec(context.Background(), `INSERT INTO agent_session_data (session_id, agent_id, context_data, capabilities) VALUES ('session-1', 'agent-1', '{}', ?)`, string(caps1))
	assert.NoError(t, err)

	caps2, _ := json.Marshal([]string{"backend", "postgres"})
	_, err = mockDB.Exec(context.Background(), `INSERT INTO agent_session_data (session_id, agent_id, context_data, capabilities) VALUES ('session-2', 'agent-2', '{}', ?)`, string(caps2))
	assert.NoError(t, err)

	// Mock Mesh
	mockMesh := new(MockTeammateMeshRouter)
	activeAgents := []AgentPresence{
		{AgentID: "agent-1", Status: "IDLE"},
		{AgentID: "agent-2", Status: "IDLE"},
	}
	mockMesh.On("GetActiveAgents", mock.Anything).Return(activeAgents, nil)

	payload := []byte(`{"job_id": "job-1"}`)
	mockMesh.On("Publish", mock.Anything, "agent:agent-2", payload).Return(nil)

	// Initialize router
	router := NewCapabilityRouter(mockDB, mockMesh)

	// Test RouteJob
	err = router.RouteJob(context.Background(), "postgres", payload)
	assert.NoError(t, err)

	mockMesh.AssertExpectations(t)
}

package mesh

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

// A simple mock for TeammateMesh to be used in tests
type MockTeammateMesh struct {
	publishedTopics []string
	publishedMsgs   [][]byte
}

func (m *MockTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.publishedTopics = append(m.publishedTopics, topic)
	m.publishedMsgs = append(m.publishedMsgs, payload)
	return nil
}

func (m *MockTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	return nil, nil
}

func (m *MockTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return true, nil
}

func (m *MockTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	return nil
}

func (m *MockTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return nil
}

func (m *MockTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	return nil, nil
}

func setupTestProvider(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	provider := db.NewSqliteProvider(sqliteDB)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS agent_mesh_messages (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			sender TEXT NOT NULL,
			recipient TEXT,
			channel TEXT NOT NULL,
			content TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)
	return provider
}

func TestMeshCoordinatorService_Publish(t *testing.T) {
	provider := setupTestProvider(t)
	defer provider.Close()

	mockMesh := &MockTeammateMesh{}
	svc := NewMeshCoordinatorService(mockMesh, provider)

	msg := &AgentMeshMessage{
		TenantID: "tenant-1",
		Sender:   "agent-1",
		Channel:  "test-channel",
		Content:  []byte(`{"hello": "world"}`),
	}

	err := svc.Publish(context.Background(), msg)
	assert.NoError(t, err)

	// verify db write
	rows, err := provider.Query(context.Background(), "SELECT id, sender, channel, content FROM agent_mesh_messages")
	require.NoError(t, err)
	defer rows.Close()

	count := 0
	for rows.Next() {
		var id, sender, channel, content string
		err := rows.Scan(&id, &sender, &channel, &content)
		assert.NoError(t, err)
		assert.NotEmpty(t, id)
		assert.Equal(t, "agent-1", sender)
		assert.Equal(t, "test-channel", channel)
		count++
	}
	assert.Equal(t, 1, count)

	// verify mock publish
	require.Len(t, mockMesh.publishedTopics, 1)
	assert.Equal(t, "test-channel", mockMesh.publishedTopics[0])
}

func TestMeshCoordinatorService_Subscribe(t *testing.T) {
	provider := setupTestProvider(t)
	defer provider.Close()

	mockMesh := &MockTeammateMesh{}
	svc := NewMeshCoordinatorService(mockMesh, provider)

	sub, err := svc.Subscribe(context.Background(), "test-channel", func(msg *AgentMeshMessage) {})
	assert.NoError(t, err)
	assert.Nil(t, sub) // mock returns nil
}

package mesh

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/repositories"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

type RouterMockTeammateMesh struct {
	activeAgents []AgentPresence
	published    map[string][][]byte
}

func (m *RouterMockTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	if m.published == nil {
		m.published = make(map[string][][]byte)
	}
	m.published[topic] = append(m.published[topic], payload)
	return nil
}

func (m *RouterMockTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	return nil, nil
}

func (m *RouterMockTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return true, nil
}

func (m *RouterMockTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	return nil
}

func (m *RouterMockTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return nil
}

func (m *RouterMockTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	return m.activeAgents, nil
}

func setupRouterTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	provider := db.NewSqliteProvider(sqliteDB)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			capabilities JSONB DEFAULT '[]',
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			last_accessed TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)
	return provider
}

func TestCapabilityRouter_RouteToCapability(t *testing.T) {
	provider := setupRouterTestDB(t)
	defer provider.Close()
	repo := repositories.NewMeshRepository(provider)

	// Seed capabilities
	// Agent 1: has "web-search"
	_, err := provider.Exec(context.Background(),
		"INSERT INTO agent_session_data (session_id, agent_id, context_data, capabilities) VALUES (?, ?, ?, ?)",
		"sess-1", "agent-1", "{}", `["web-search", "image-gen"]`)
	require.NoError(t, err)

	// Agent 2: has "web-search"
	_, err = provider.Exec(context.Background(),
		"INSERT INTO agent_session_data (session_id, agent_id, context_data, capabilities) VALUES (?, ?, ?, ?)",
		"sess-2", "agent-2", "{}", `["web-search"]`)
	require.NoError(t, err)

	// Agent 3: has "db-query"
	_, err = provider.Exec(context.Background(),
		"INSERT INTO agent_session_data (session_id, agent_id, context_data, capabilities) VALUES (?, ?, ?, ?)",
		"sess-3", "agent-3", "{}", `["db-query"]`)
	require.NoError(t, err)

	t.Run("Successfully route to active and capable agent", func(t *testing.T) {
		mockMesh := &RouterMockTeammateMesh{
			activeAgents: []AgentPresence{
				{AgentID: "agent-1", Status: "IDLE"},
				{AgentID: "agent-3", Status: "BUSY"},
			},
		}
		router := NewCapabilityRouter(mockMesh, repo)

		payload := []byte("test-job")
		selectedAgent, err := router.RouteToCapability(context.Background(), "web-search", payload)

		assert.NoError(t, err)
		assert.Equal(t, "agent-1", selectedAgent)
		assert.Contains(t, mockMesh.published, "agent:job:agent-1")
		assert.Equal(t, payload, mockMesh.published["agent:job:agent-1"][0])
	})

	t.Run("Fail when no active agent has capability", func(t *testing.T) {
		mockMesh := &RouterMockTeammateMesh{
			activeAgents: []AgentPresence{
				{AgentID: "agent-3", Status: "IDLE"},
			},
		}
		router := NewCapabilityRouter(mockMesh, repo)

		_, err := router.RouteToCapability(context.Background(), "web-search", []byte("test"))
		assert.Error(t, err)
		assert.Contains(t, err.Error(), "no active agents currently possess the required capability")
	})

	t.Run("Fail when no agent has capability at all", func(t *testing.T) {
		mockMesh := &RouterMockTeammateMesh{
			activeAgents: []AgentPresence{
				{AgentID: "agent-1", Status: "IDLE"},
			},
		}
		router := NewCapabilityRouter(mockMesh, repo)

		_, err := router.RouteToCapability(context.Background(), "flight-booking", []byte("test"))
		assert.Error(t, err)
		assert.Contains(t, err.Error(), "no agents found with capability: flight-booking")
	})

	t.Run("Fail when no agents are active", func(t *testing.T) {
		mockMesh := &RouterMockTeammateMesh{
			activeAgents: []AgentPresence{},
		}
		router := NewCapabilityRouter(mockMesh, repo)

		_, err := router.RouteToCapability(context.Background(), "web-search", []byte("test"))
		assert.Error(t, err)
		assert.Contains(t, err.Error(), "no active agents available in the mesh")
	})
}

package autodream

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

type KairosMockWorkerLLMClient struct {
	embeddings [][]float32
}

func (m *KairosMockWorkerLLMClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "mocked reasoning", nil
}

func (m *KairosMockWorkerLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	emb := []float32{0.1, 0.2, 0.3}
	m.embeddings = append(m.embeddings, emb)
	return emb, nil
}

func setupKairosTestProvider(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	provider := db.NewSqliteProvider(sqliteDB)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			payload TEXT
		)
	`)
	require.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS agent_mesh_messages (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			sender TEXT NOT NULL,
			channel TEXT NOT NULL,
			content TEXT NOT NULL
		)
	`)
	require.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS agent_memory_embeddings (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			tenant_id TEXT NOT NULL DEFAULT '',
			agent_id TEXT NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding TEXT
		)
	`)
	require.NoError(t, err)

	return provider
}

func TestKairosAutoDreamWorker_RunConsolidation(t *testing.T) {
	provider := setupKairosTestProvider(t)
	defer provider.Close()

	mockLLM := &KairosMockWorkerLLMClient{}
	worker := NewKairosAutoDreamWorker(provider, mockLLM)

	// insert mock task
	_, err := provider.Exec(context.Background(), `
		INSERT INTO shared_tasks (id, title, status, payload) VALUES ('task-1', 'Task 1', 'COMPLETED', '{}')
	`)
	require.NoError(t, err)

	// insert mock message
	_, err = provider.Exec(context.Background(), `
		INSERT INTO agent_mesh_messages (id, tenant_id, sender, channel, content) VALUES ('msg-1', 'tenant-1', 'agent-1', 'ch-1', 'msg content')
	`)
	require.NoError(t, err)

	err = worker.RunConsolidation(context.Background())
	assert.NoError(t, err)

	// verify memory insertions
	rows, err := provider.Query(context.Background(), "SELECT agent_id, memory_type FROM agent_memory_embeddings")
	require.NoError(t, err)
	defer rows.Close()

	memories := 0
	hasTaskMem := false
	hasMsgMem := false
	for rows.Next() {
		var agentID, memType string
		err := rows.Scan(&agentID, &memType)
		assert.NoError(t, err)
		if memType == "task_memory" {
			hasTaskMem = true
		} else if memType == "mesh_message" {
			hasMsgMem = true
		}
		memories++
	}
	assert.Equal(t, 2, memories)
	assert.True(t, hasTaskMem)
	assert.True(t, hasMsgMem)

	// verify task updated
	row := provider.QueryRow(context.Background(), "SELECT status FROM shared_tasks WHERE id = 'task-1'")
	var status string
	err = row.Scan(&status)
	assert.NoError(t, err)
	assert.Equal(t, "CONSOLIDATED", status)

	// verify message deleted
	row = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_mesh_messages WHERE id = 'msg-1'")
	var count int
	err = row.Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 0, count)
}

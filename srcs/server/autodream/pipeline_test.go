package autodream

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

type MockLLMClient struct {
	err error
}

func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.err != nil {
		return nil, m.err
	}
	// Return a vector of size 1536
	return make([]float32, 1536), nil
}

func setupTestDB(t *testing.T) db.Provider {
	pool := db.NewTestProvider(t)
	return pool
}

func TestAutoDreamPipeline_ProcessCompletedTasks(t *testing.T) {
	pool := setupTestDB(t)
	ctx := context.Background()

	// Ensure required tables exist
	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL
		)
	`)
	assert.NoError(t, err)

	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	// Insert test data
	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks_decomposition (id, organization_id, title, description, status)
		VALUES ('task-1', 'org-1', 'Test Task', 'Test Description', 'COMPLETED')
	`)
	assert.NoError(t, err)

	// Task 2 with no description but should fallback to title
	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks_decomposition (id, organization_id, title, status)
		VALUES ('task-2', 'org-1', 'Title Only Task', 'COMPLETED')
	`)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(pool, &MockLLMClient{})

	err = pipeline.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 2, count)

	// Verify task-1 content
	var content string
	err = pool.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE task_id = 'task-1'").Scan(&content)
	assert.NoError(t, err)
	assert.Equal(t, "Test Description", content)

	// Verify task-2 content
	err = pool.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE task_id = 'task-2'").Scan(&content)
	assert.NoError(t, err)
	assert.Equal(t, "Title Only Task", content)
}

func TestAutoDreamPipeline_ProcessCompletedTasks_EmbeddingErrorFallback(t *testing.T) {
	pool := setupTestDB(t)
	ctx := context.Background()

	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL
		)
	`)
	assert.NoError(t, err)

	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks_decomposition (id, organization_id, title, description, status)
		VALUES ('task-3', 'org-1', 'Test Task', 'Test Description', 'COMPLETED')
	`)
	assert.NoError(t, err)

	// Will cause LLM error and should NOT process, skipping for retry
	pipeline := NewAutoDreamPipeline(pool, &MockLLMClient{err: errors.New("llm error")})

	err = pipeline.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE task_id = 'task-3'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 0, count)
}

func TestAutoDreamPipeline_ProcessCompletedTasks_QueryError(t *testing.T) {
	mockDB := &mockPGProvider{
		queryErr: errors.New("query error"),
	}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessCompletedTasks(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestAutoDreamPipeline_ProcessCompletedTasks_ScanError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanErr: errors.New("scan error"),
		},
	}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessCompletedTasks(context.Background())
	if err != nil {
		t.Fatalf("expected no error but scan errors should be skipped, got %v", err)
	}
}

func TestAutoDreamPipeline_ProcessCompletedTasks_RowError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 0,
			errErr:  errors.New("row error"),
		},
	}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessCompletedTasks(context.Background())
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestAutoDreamPipeline_ProcessCompletedTasks_EmbeddingError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"org-id",
				"content",
			},
		},
	}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{err: errors.New("embedding error")})
	err := pipeline.ProcessCompletedTasks(context.Background())
	if err != nil {
		t.Fatalf("expected no error but skipped embedding error, got %v", err)
	}
}

func TestAutoDreamPipeline_ProcessCompletedTasks_InsertError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"org-id",
				"content",
			},
		},
		execErr: errors.New("insert error"),
	}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessCompletedTasks(context.Background())
	if err != nil {
		t.Fatalf("expected no error but logged insert error, got %v", err)
	}
}

func TestAutoDreamPipeline_ProcessCompletedTasks_EmptyContent(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"org-id",
				"", // empty content
			},
		},
	}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessCompletedTasks(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestAutoDreamPipeline_ProcessCompletedSwarmTasks(t *testing.T) {
	pool := setupTestDB(t)
	ctx := context.Background()

	// Ensure tables exist
	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL
		)
	`)
	assert.NoError(t, err)

	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	// Insert test data
	_, err = pool.Exec(ctx, `
		INSERT INTO swarm_tasks (id, organization_id, title, payload, status)
		VALUES ('swarm-1', 'org-1', 'Test Swarm Task', '{"key":"value"}', 'COMPLETED')
	`)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(pool, &MockLLMClient{})

	err = pipeline.ProcessCompletedSwarmTasks(ctx)
	assert.NoError(t, err)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'swarm_task'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	// Verify swarm_tasks status updated to ARCHIVED
	var status string
	err = pool.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = 'swarm-1'").Scan(&status)
	assert.NoError(t, err)
	assert.Equal(t, "ARCHIVED", status)
}

type mockTxProvider struct {
	db.Provider
	beginErr error
}

func (m *mockTxProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, m.beginErr
}

func TestAutoDreamPipeline_ProcessCompletedSwarmTasks_Errors(t *testing.T) {
	mockDB := &mockTxProvider{beginErr: errors.New("begin error")}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessCompletedSwarmTasks(context.Background())
	assert.Error(t, err)
}

func TestAutoDreamPipeline_ProcessSessionLogs(t *testing.T) {
	pool := setupTestDB(t)
	ctx := context.Background()

	// Ensure tables exist
	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	// Insert test data
	_, err = pool.Exec(ctx, `
		INSERT INTO agent_session_data (session_id, agent_id, context_data)
		VALUES ('session-1', 'agent-1', 'This is a test session log content.')
	`)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(pool, &MockLLMClient{})

	err = pipeline.ProcessSessionLogs(ctx)
	assert.NoError(t, err)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'session_log'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	// Verify session deleted
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 'session-1'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 0, count)
}

func TestAutoDreamPipeline_ProcessSessionLogs_Errors(t *testing.T) {
	mockDB := &mockTxProvider{beginErr: errors.New("begin error")}
	pipeline := NewAutoDreamPipeline(mockDB, &MockEmbeddingClient{})
	err := pipeline.ProcessSessionLogs(context.Background())
	assert.Error(t, err)
}

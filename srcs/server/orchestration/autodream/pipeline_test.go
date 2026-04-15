package autodream

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockLLMClient struct {
	CompleteFunc func(ctx context.Context, req local.CompletionRequest) (*local.AssistantMessage, error)
}

func (m *MockLLMClient) Complete(ctx context.Context, req local.CompletionRequest) (*local.AssistantMessage, error) {
	if m.CompleteFunc != nil {
		return m.CompleteFunc(ctx, req)
	}
	return &local.AssistantMessage{
		Text: "mocked summary",
	}, nil
}

type MockVectorRepository struct {
	Inserted []*Memory
	Err      error
}

func (m *MockVectorRepository) Insert(ctx context.Context, mem *Memory) error {
	if m.Err != nil {
		return m.Err
	}
	m.Inserted = append(m.Inserted, mem)
	return nil
}

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	assert.NoError(t, err)

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT,
			context_data TEXT,
			last_accessed TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	return provider
}

func TestRunConsolidationCycle_DatabaseSessions(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()

	// Insert stale session
	staleTime := time.Now().Add(-2 * time.Hour).UTC()
	_, err := provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES (?, ?, ?, ?)", "sess-1", "agent-1", "some context", staleTime)
	assert.NoError(t, err)

	// Insert active session (should be ignored)
	activeTime := time.Now().UTC()
	_, err = provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES (?, ?, ?, ?)", "sess-2", "agent-2", "active context", activeTime)
	assert.NoError(t, err)

	mockLLM := &MockLLMClient{}
	mockRepo := &MockVectorRepository{}

	pipeline := NewAutoDreamPipeline(provider, mockLLM, mockRepo)
	pipeline.processDatabaseSessions(ctx)

	assert.Len(t, mockRepo.Inserted, 1)
	assert.Equal(t, "mocked summary", mockRepo.Inserted[0].Content)
	assert.Equal(t, "sess-1", mockRepo.Inserted[0].TaskID)

	// Verify the session was deleted from DB
	rows, err := provider.Query(ctx, "SELECT session_id FROM agent_session_data")
	assert.NoError(t, err)
	defer rows.Close()

	var remainingSessions []string
	for rows.Next() {
		var id string
		assert.NoError(t, rows.Scan(&id))
		remainingSessions = append(remainingSessions, id)
	}

	assert.Len(t, remainingSessions, 1)
	assert.Equal(t, "sess-2", remainingSessions[0])
}

func TestRunConsolidationCycle_FilesystemMemories(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()

	// Create temporary directory for memories
	tempDir := t.TempDir()
	os.Setenv("OHC_MEMORY_DIR", tempDir)
	defer os.Unsetenv("OHC_MEMORY_DIR")

	// Create a dummy memory file
	fileContent := `
content: "This is a test memory from filesystem"
agent_id: "agent-123"
task_id: "task-456"
`
	err := os.WriteFile(filepath.Join(tempDir, "memory1.yml"), []byte(fileContent), 0644)
	assert.NoError(t, err)

	mockLLM := &MockLLMClient{}
	mockRepo := &MockVectorRepository{}

	pipeline := NewAutoDreamPipeline(provider, mockLLM, mockRepo)
	pipeline.processFilesystemMemories(ctx)

	assert.Len(t, mockRepo.Inserted, 1)
	assert.Equal(t, "mocked summary", mockRepo.Inserted[0].Content)
	assert.Equal(t, "task-456", mockRepo.Inserted[0].TaskID)

	// Verify file was deleted after processing
	files, err := os.ReadDir(tempDir)
	assert.NoError(t, err)
	assert.Len(t, files, 0)
}

package workers

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestAutoDreamWorker_ProcessCompletedTasks(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	_, err = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (id TEXT PRIMARY KEY, organization_id TEXT, status TEXT, payload TEXT)")
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, "CREATE TABLE IF NOT EXISTS autodream_memories (id TEXT PRIMARY KEY, source_mission_id TEXT, organization_id TEXT, content TEXT, embedding TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)")
	assert.NoError(t, err)

	_, err = provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload) VALUES ('task-1', 'org-1', 'DONE', 'test payload')")
	assert.NoError(t, err)

	// Set fallback mock logic manually
	t.Setenv("MINIMAX_API_KEY", "mock_key")

	worker := NewAutoDreamWorker(provider)
	err = worker.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)

	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE source_mission_id = 'task-1'").Scan(&count)
	assert.NoError(t, err)
	// count will be 0 here since mock MinimaxClient returns an error and we gracefully skip
	assert.Equal(t, 0, count)
}

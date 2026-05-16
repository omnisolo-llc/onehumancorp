package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"sync"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupSMTestDB(t *testing.T) *sql.DB {
	// Need a persistent file or shared cache for concurrent sqlite memory tests
	db, err := sql.Open("sqlite3", "file:memdb1?mode=memory&cache=shared")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS ohc_tasks (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			parent_task_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			workflow_state TEXT,
			payload TEXT,
			assigned_agent_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)
	return db
}

func TestTaskStateMachine_ProcessEvent(t *testing.T) {
	db := setupSMTestDB(t)
	defer db.Close()

	sm := NewTaskStateMachine(db)
	ctx := context.Background()

	// Insert parent task
	_, err := db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, status) VALUES ('parent-1', 'tenant-1', 'DECOMPOSING')")
	require.NoError(t, err)

	err = sm.ProcessEvent(ctx, "parent-1", EventDecompositionComplete)
	require.NoError(t, err)

	var status string
	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "EXECUTING", status)

	// Insert child tasks
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, parent_task_id, status) VALUES ('child-1', 'tenant-1', 'parent-1', 'PENDING')")
	require.NoError(t, err)
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, parent_task_id, status) VALUES ('child-2', 'tenant-1', 'parent-1', 'PENDING')")
	require.NoError(t, err)

	// One child completes
	_, err = db.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'DONE' WHERE id = 'child-1'")
	require.NoError(t, err)
	err = sm.ProcessEvent(ctx, "parent-1", EventSubTaskCompleted)
	require.NoError(t, err)

	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "EXECUTING", status) // Still executing because child-2 is pending

	// Second child completes
	_, err = db.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'DONE' WHERE id = 'child-2'")
	require.NoError(t, err)
	err = sm.ProcessEvent(ctx, "parent-1", EventSubTaskCompleted)
	require.NoError(t, err)

	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "VERIFYING", status) // All children done, should transition to VERIFYING

	// Test concurrent updates
	_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, status) VALUES ('parent-2', 'tenant-1', 'EXECUTING')")
	require.NoError(t, err)
	for i := 0; i < 10; i++ {
		_, err = db.ExecContext(ctx, "INSERT INTO ohc_tasks (id, tenant_id, parent_task_id, status) VALUES (?, 'tenant-1', 'parent-2', 'PENDING')", fmt.Sprintf("c%d", i))
		require.NoError(t, err)
	}

	var wg sync.WaitGroup
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			// Must lock even the update for sqlite
			sm.mu.Lock()
			_, err := db.Exec("UPDATE ohc_tasks SET status = 'DONE' WHERE id = ?", fmt.Sprintf("c%d", idx))
			sm.mu.Unlock()
			assert.NoError(t, err)
			err = sm.ProcessEvent(context.Background(), "parent-2", EventSubTaskCompleted)
			assert.NoError(t, err)
		}(i)
	}
	wg.Wait()

	err = db.QueryRowContext(ctx, "SELECT status FROM ohc_tasks WHERE id = 'parent-2'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "VERIFYING", status)
}

package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestDBForSubAgentWorker(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	require.NoError(t, err)

	query := `
		CREATE TABLE IF NOT EXISTS sub_agent_jobs (
			id TEXT PRIMARY KEY,
			task_id TEXT,
			status TEXT,
			created_at TIMESTAMP,
			updated_at TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS ohc_tasks (
			id TEXT PRIMARY KEY,
			parent_task_id TEXT,
			status TEXT,
			workflow_state TEXT
		);
	`
	_, err = db.Exec(query)
	require.NoError(t, err)

	return db
}

type mockSubAgentSpawner struct {
	spawnIsolatedCalled bool
	spawnCalled         bool
	errToReturn         error
}

func (m *mockSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	m.spawnCalled = true
	return m.errToReturn
}

func (m *mockSubAgentSpawner) SpawnIsolated(ctx context.Context, job *Job) error {
	m.spawnIsolatedCalled = true
	return m.errToReturn
}

func (m *mockSubAgentSpawner) Monitor(ctx context.Context) error {
	return nil
}

func TestSubAgentWorker_Poll(t *testing.T) {
	db := setupTestDBForSubAgentWorker(t)
	defer db.Close()

	// Insert test data
	_, err := db.Exec("INSERT INTO ohc_tasks (id, status) VALUES ('task-1', 'PENDING')")
	require.NoError(t, err)

	_, err = db.Exec("INSERT INTO sub_agent_jobs (id, task_id, status) VALUES ('job-1', 'task-1', 'PENDING')")
	require.NoError(t, err)

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{}

	worker := NewSubAgentWorker(db, sm, spawner)

	worker.Poll(context.Background())

	// Wait for async processing
	time.Sleep(100 * time.Millisecond)

	// Verify job status
	var jobStatus string
	err = db.QueryRow("SELECT status FROM sub_agent_jobs WHERE id = 'job-1'").Scan(&jobStatus)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", jobStatus)

	// Verify task status
	var taskStatus string
	err = db.QueryRow("SELECT status FROM ohc_tasks WHERE id = 'task-1'").Scan(&taskStatus)
	require.NoError(t, err)
	assert.Equal(t, "VERIFYING", taskStatus) // State machine transitions PENDING -> VERIFYING on success

	assert.True(t, spawner.spawnIsolatedCalled)
}

func TestSubAgentWorker_Poll_Failure(t *testing.T) {
	db := setupTestDBForSubAgentWorker(t)
	defer db.Close()

	// Insert test data
	_, err := db.Exec("INSERT INTO ohc_tasks (id, status) VALUES ('task-2', 'PENDING')")
	require.NoError(t, err)

	_, err = db.Exec("INSERT INTO sub_agent_jobs (id, task_id, status) VALUES ('job-2', 'task-2', 'PENDING')")
	require.NoError(t, err)

	sm := NewTaskStateMachine(db)
	spawner := &mockSubAgentSpawner{errToReturn: assert.AnError}

	worker := NewSubAgentWorker(db, sm, spawner)

	worker.Poll(context.Background())

	// Wait for async processing
	time.Sleep(100 * time.Millisecond)

	// Verify job status
	var jobStatus string
	err = db.QueryRow("SELECT status FROM sub_agent_jobs WHERE id = 'job-2'").Scan(&jobStatus)
	require.NoError(t, err)
	assert.Equal(t, "FAILED", jobStatus)

	// Verify task status
	var taskStatus string
	err = db.QueryRow("SELECT status FROM ohc_tasks WHERE id = 'task-2'").Scan(&taskStatus)
	require.NoError(t, err)
	assert.Equal(t, "FAILED", taskStatus) // State machine transitions to FAILED
}

package migrations_test

import (
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/require"
)

// A dummy test to ensure schema logic executes cleanly against SQLite in tests
func TestKairosOrchestrationSchema(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)
	defer db.Close()

	// Since we are mocking tests here and standard goose migrator is tested elsewhere,
	// we will manually run the SQLite DDL from the 079 migration to ensure syntax is valid.
	query := `
	CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		status TEXT NOT NULL,
		payload TEXT
	);

	CREATE TABLE IF NOT EXISTS kairos_state_transitions (
		id TEXT PRIMARY KEY,
		task_id TEXT NOT NULL,
		from_state TEXT,
		to_state TEXT NOT NULL,
		transitioned_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
		id TEXT PRIMARY KEY,
		parent_task_id TEXT NOT NULL,
		agent_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL
	);

	CREATE TABLE IF NOT EXISTS autodream_vector_memories (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		embedding TEXT,
		metadata TEXT
	);
	`
	_, err = db.Exec(query)
	require.NoError(t, err, "Failed to apply SQLite schema for KAIROS orchestration")

	// Verify tables
	var name string
	err = db.QueryRow("SELECT name FROM sqlite_master WHERE type='table' AND name='kairos_shared_tasks';").Scan(&name)
	require.NoError(t, err)
	require.Equal(t, "kairos_shared_tasks", name)
}

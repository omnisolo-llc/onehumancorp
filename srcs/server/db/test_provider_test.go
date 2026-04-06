package db

import (
	"database/sql"
	"testing"

	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

// Helper for other tests to initialize a test provider
func NewTestProviderForOtherPackages(t *testing.T) Provider {
	t.Helper()
	db, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	t.Cleanup(func() { db.Close() })

	_, err = db.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			parent_plan_id TEXT,
			title TEXT,
			payload JSONB,
			status TEXT,
			priority INTEGER,
			locked_until DATETIME,
			created_at DATETIME,
			updated_at DATETIME,
			agent_id TEXT
		);
		CREATE TABLE task_dependencies (
			task_id TEXT,
			depends_on_task_id TEXT,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)
	require.NoError(t, err)

	return NewSqliteProvider(db)
}

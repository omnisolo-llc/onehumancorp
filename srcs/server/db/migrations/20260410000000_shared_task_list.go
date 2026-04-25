package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSharedTaskList, downSharedTaskList)
}

func upSharedTaskList(ctx context.Context, tx *sql.Tx) error {
	var isSqlite bool
	_, err := tx.ExecContext(ctx, "SAVEPOINT check_sqlite")
	if err == nil {
		var v string
		err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&v)
		if err == nil {
			isSqlite = true
			_ = tx.ExecContext(ctx, "RELEASE SAVEPOINT check_sqlite")
		} else {
			_ = tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT check_sqlite")
		}
	}

	if isSqlite {
		_, err = tx.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS shared_tasks (
				id TEXT PRIMARY KEY,
				tenant_id TEXT NOT NULL,
				title TEXT NOT NULL,
				description TEXT,
				status TEXT NOT NULL CHECK (status IN ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'FAILED')),
				priority TEXT NOT NULL CHECK (priority IN ('P0', 'P1', 'P2')),
				agent_id TEXT,
				created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
				updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);
			CREATE TABLE IF NOT EXISTS task_dependencies (
				task_id TEXT NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
				depends_on_task_id TEXT NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
				tenant_id TEXT NOT NULL,
				PRIMARY KEY (task_id, depends_on_task_id)
			);
		`)
		return err
	}

	// PostgreSQL
	_, err = tx.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL CHECK (status IN ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'FAILED')),
			priority TEXT NOT NULL CHECK (priority IN ('P0', 'P1', 'P2')),
			agent_id TEXT,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
		ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
		CREATE POLICY "tenant_isolation_policy" ON shared_tasks
			USING (tenant_id = current_setting('app.current_tenant_id', true));

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
			depends_on_task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
			tenant_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
		ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
		CREATE POLICY "tenant_isolation_policy" ON task_dependencies
			USING (tenant_id = current_setting('app.current_tenant_id', true));
	`)
	return err
}

func downSharedTaskList(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, `
		DROP TABLE IF EXISTS task_dependencies;
		DROP TABLE IF EXISTS shared_tasks;
	`)
	return err
}

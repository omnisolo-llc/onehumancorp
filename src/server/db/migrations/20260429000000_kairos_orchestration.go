package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKairosOrchestration, downKairosOrchestration)
}

func upKairosOrchestration(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}

		query := `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload JSONB,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
		);

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id UUID REFERENCES shared_tasks(id) ON DELETE CASCADE,
			depends_on_task_id UUID REFERENCES shared_tasks(id) ON DELETE CASCADE,
			PRIMARY KEY (task_id, depends_on_task_id)
		);

		CREATE TABLE IF NOT EXISTS agent_memories (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			organization_id VARCHAR NOT NULL,
			content TEXT NOT NULL,
			embedding vector(1536),
			created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
		);
		`
		_, err = tx.ExecContext(ctx, query)
		return err
	}

	query := `
	CREATE TABLE IF NOT EXISTS shared_tasks (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		agent_id TEXT,
		priority TEXT NOT NULL DEFAULT 'P2',
		payload TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS task_dependencies (
		task_id TEXT,
		depends_on_task_id TEXT,
		PRIMARY KEY (task_id, depends_on_task_id),
		FOREIGN KEY (task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE,
		FOREIGN KEY (depends_on_task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE
	);

	CREATE TABLE IF NOT EXISTS agent_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downKairosOrchestration(ctx context.Context, tx *sql.Tx) error {
	// Let's not drop tables just in case, but normally we would:
	// _, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS task_dependencies, shared_tasks, agent_memories;")
	// return err
	return nil
}

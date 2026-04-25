package db

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upCloudSearchIndex, downCloudSearchIndex)
}

func upCloudSearchIndex(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS cloud_search_index (
				id VARCHAR(255) PRIMARY KEY,
				tenant_id VARCHAR(255) NOT NULL,
				title VARCHAR(255) NOT NULL,
				content TEXT NOT NULL
			);
		`)
		if err != nil {
			return fmt.Errorf("failed to create cloud_search_index: %w", err)
		}

		_, err = tx.ExecContext(ctx, `
			ALTER TABLE cloud_search_index ENABLE ROW LEVEL SECURITY;
		`)
		if err != nil {
			return fmt.Errorf("failed to enable rls on cloud_search_index: %w", err)
		}

		_, err = tx.ExecContext(ctx, `
			DROP POLICY IF EXISTS tenant_isolation_policy ON cloud_search_index;
		`)
		if err != nil {
			return fmt.Errorf("failed to drop old policy on cloud_search_index: %w", err)
		}

		_, err = tx.ExecContext(ctx, `
			CREATE POLICY tenant_isolation_policy ON cloud_search_index USING (tenant_id = current_setting('app.current_tenant', true));
		`)
		if err != nil {
			return fmt.Errorf("failed to create policy on cloud_search_index: %w", err)
		}

		return nil
	}

	// SQLite migrations
	_, err = tx.ExecContext(ctx, `
		CREATE VIRTUAL TABLE IF NOT EXISTS local_search_index USING fts5(
			id UNINDEXED,
			title,
			content
		);
	`)
	return err
}

func downCloudSearchIndex(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS cloud_search_index CASCADE")
		return err
	}

	_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS local_search_index")
	return err
}

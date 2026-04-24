package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upPendingApprovalQueue20260428010000, downPendingApprovalQueue20260428010000)
}

func upPendingApprovalQueue20260428010000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS pending_approvals (
				id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
				tenant_id VARCHAR NOT NULL,
				organization_id VARCHAR NOT NULL,
				agent_id VARCHAR NOT NULL,
				task_id VARCHAR NOT NULL,
				action TEXT NOT NULL,
				action_risk VARCHAR NOT NULL DEFAULT 'LOW',
				status VARCHAR NOT NULL DEFAULT 'PENDING',
				created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
				updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
			)
		`)
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, `CREATE INDEX IF NOT EXISTS idx_pending_approvals_tenant_status ON pending_approvals(tenant_id, status)`)
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, `ALTER TABLE pending_approvals ENABLE ROW LEVEL SECURITY`)
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, `CREATE POLICY tenant_isolation_policy_pending_approvals ON pending_approvals USING (tenant_id = current_setting('app.current_tenant_id', true))`)
		if err != nil && !strings.Contains(err.Error(), "already exists") {
			return err
		}

		return nil
	}

	_, err = tx.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS pending_approvals (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			organization_id TEXT NOT NULL,
			agent_id TEXT NOT NULL,
			task_id TEXT NOT NULL,
			action TEXT NOT NULL,
			action_risk TEXT NOT NULL DEFAULT 'LOW',
			status TEXT NOT NULL DEFAULT 'PENDING',
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		return err
	}

	_, err = tx.ExecContext(ctx, `CREATE INDEX IF NOT EXISTS idx_pending_approvals_tenant_status ON pending_approvals(tenant_id, status)`)
	return err
}

func downPendingApprovalQueue20260428010000(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS pending_approvals")
	return err
}

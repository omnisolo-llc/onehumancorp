package workers

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestCompetitorAuditWorker(t *testing.T) {
	// 1. Setup in-memory sqlite provider
	dbConn, err := sql.Open("sqlite", ":memory:")
	assert.NoError(t, err)

    // Run migration
    _, err = dbConn.Exec(`
    CREATE TABLE IF NOT EXISTS competitor_metrics (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        competitor_name TEXT NOT NULL,
        metric_type TEXT NOT NULL,
        metric_value TEXT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );`)
    assert.NoError(t, err)

	pool := db.NewSqliteProvider(dbConn)


	// 2. Instantiate the worker
	worker := NewCompetitorAuditWorker(pool)

	// 3. Run audit manually
	ctx := context.Background()
	worker.runAudit(ctx)

	// 4. Verify DB insertion
	rows, err := pool.Query(ctx, "SELECT competitor_name FROM competitor_metrics")
	assert.NoError(t, err)
	defer rows.Close()

	var names []string
	for rows.Next() {
		var name string
		err := rows.Scan(&name)
		assert.NoError(t, err)
		names = append(names, name)
	}
	assert.ElementsMatch(t, []string{"Claude Code", "OpenClaw", "Replit Agent"}, names)


}

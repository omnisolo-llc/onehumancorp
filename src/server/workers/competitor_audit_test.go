package workers

import (
	"context"
	"database/sql"
	"testing"
	"time"
	"errors"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

// mockProvider implements db.Provider for testing error paths
type mockProvider struct {
	db.Provider
	beginErr  error
	execErr   error
	commitErr error
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	if m.beginErr != nil {
		return nil, m.beginErr
	}
	return &mockTx{execErr: m.execErr, commitErr: m.commitErr}, nil
}

type mockTx struct {
	db.Tx
	execErr   error
	commitErr error
}

func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	return 0, nil
}

func (m *mockTx) Commit(ctx context.Context) error {
	return m.commitErr
}

func (m *mockTx) Rollback(ctx context.Context) error {
	return nil
}


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

func TestCompetitorAuditWorker_Start(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	assert.NoError(t, err)
	pool := db.NewSqliteProvider(dbConn)
	worker := NewCompetitorAuditWorker(pool)

	ctx, cancel := context.WithCancel(context.Background())

	// Start in a goroutine
	done := make(chan struct{})
	go func() {
		worker.Start(ctx)
		close(done)
	}()

	// Give it a tiny bit of time to start and do its first run, then cancel
	time.Sleep(10 * time.Millisecond)
	cancel()

	// Wait for Start to finish
	<-done
}

func TestCompetitorAuditWorker_StartLoop(t *testing.T) {
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
	worker := NewCompetitorAuditWorker(pool)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

    go func() {
        // give it 100ms to run
        time.Sleep(100 * time.Millisecond)
        cancel()
    }()

	worker.Start(ctx) // This will block for an hour if not canceled. The test ensures that cancellation unblocks it.
}

func TestCompetitorAuditWorker_Errors(t *testing.T) {
	ctx := context.Background()

	t.Run("BeginError", func(t *testing.T) {
		m := &mockProvider{beginErr: errors.New("begin error")}
		worker := NewCompetitorAuditWorker(m)
		worker.runAudit(ctx) // Should return early, logs error
	})

	t.Run("ExecError", func(t *testing.T) {
		m := &mockProvider{execErr: errors.New("exec error")}
		worker := NewCompetitorAuditWorker(m)
		worker.runAudit(ctx) // Should rollback and return, logs error
	})

	t.Run("CommitError", func(t *testing.T) {
		m := &mockProvider{commitErr: errors.New("commit error")}
		worker := NewCompetitorAuditWorker(m)
		worker.runAudit(ctx) // Should fail commit and return, logs error
	})
}

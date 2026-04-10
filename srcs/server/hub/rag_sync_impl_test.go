package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

// sqlRowsAdapter wraps *sql.Rows to implement db.Rows
type sqlRowsAdapter struct {
	*sql.Rows
}

func (r *sqlRowsAdapter) Close() {
	r.Rows.Close()
}

// sqlRowAdapter wraps *sql.Row to implement db.Row
type sqlRowAdapter struct {
	*sql.Row
}

// testDBProvider implements db.Provider for testing
type testDBProvider struct {
	db *sql.DB
}

func newTestDBProvider(t *testing.T) *testDBProvider {
	db, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	return &testDBProvider{db: db}
}

func (p *testDBProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	res, err := p.db.ExecContext(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	return res.RowsAffected()
}

func (p *testDBProvider) Query(ctx context.Context, query string, args ...any) (db.Rows, error) {
	rows, err := p.db.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	return &sqlRowsAdapter{rows}, nil
}

func (p *testDBProvider) QueryRow(ctx context.Context, query string, args ...any) db.Row {
	row := p.db.QueryRowContext(ctx, query, args...)
	return &sqlRowAdapter{row}
}

func (p *testDBProvider) Close() {
	p.db.Close()
}

func (p *testDBProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil // not used in this test
}

func (p *testDBProvider) IsSQLite() bool {
	return true
}

func (p *testDBProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}


func TestSQLRAGSyncService(t *testing.T) {
	provider := newTestDBProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Initial schema setup for test
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT,
			last_sync_at TIMESTAMP,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	svc := NewSQLRAGSyncService(provider)

	// Test 1: Fetch pending (should be empty)
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Empty(t, pending)

	// Insert some test data
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'memory 1', 'pending'), ('2', 'memory 2', NULL), ('3', 'memory 3', 'synced')
	`)
	require.NoError(t, err)

	// Test 2: Fetch pending (should get 1 and 2)
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 2)

	// Test 3: MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	require.NoError(t, err)

	// Verify only 2 is pending now
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 1)
	assert.Equal(t, "2", pending[0].ID)

	// Test 4: ProcessIncomingSync (insert and update)
	now := time.Now().Truncate(time.Millisecond)
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "updated memory 3", LastSyncAt: now}, // update existing
		{ID: "4", Context: "new memory 4", LastSyncAt: now},     // insert new
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	require.NoError(t, err)

	// Verify update
	var content, status string
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '3'").Scan(&content, &status)
	require.NoError(t, err)
	assert.Equal(t, "updated memory 3", content)
	assert.Equal(t, "synced", status)

	// Verify insert
	err = provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '4'").Scan(&content, &status)
	require.NoError(t, err)
	assert.Equal(t, "new memory 4", content)
	assert.Equal(t, "synced", status)
}

package hybrid_sync

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

// newTestDB creates a new in-memory SQLite DB wrapper for testing.
func newTestDB(t *testing.T) *db.DB {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqlDB.Close()
	})

	provider := db.NewSqliteProvider(sqlDB)
	// database.go has `func New(ctx context.Context, provider Provider) (*DB, error)`
	return &db.DB{Provider: provider}
}

func TestSQLRAGSyncService(t *testing.T) {
	ctx := context.Background()

	dbConn := newTestDB(t)

	// Setup table
	_, err := dbConn.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BYTEA,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	require.NoError(t, err)

	svc := NewSQLRAGSyncService(dbConn)

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		// Insert test data
		_, err = dbConn.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('mem1', 'ctx1', 'pending')`)
		require.NoError(t, err)
		_, err = dbConn.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('mem2', 'ctx2', 'synced')`)
		require.NoError(t, err)

		records, err := svc.FetchPendingSyncs(ctx, 10)
		assert.NoError(t, err)
		assert.Len(t, records, 1)
		assert.Equal(t, "mem1", records[0].ID)
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := svc.MarkSynced(ctx, []string{"mem1"})
		assert.NoError(t, err)

		var status string
		err = dbConn.QueryRow(ctx, `SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem1'`).Scan(&status)
		assert.NoError(t, err)
		assert.Equal(t, string(SyncStatusSynced), status)
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		records := []RAGSyncRecord{
			{ID: "mem3", Context: "ctx3", Vector: []byte{1, 2, 3}},
			{ID: "mem1", Context: "ctx1_updated"},
		}

		err := svc.ProcessIncomingSync(ctx, records)
		assert.NoError(t, err)

		var context string
		err = dbConn.QueryRow(ctx, `SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'mem3'`).Scan(&context)
		assert.NoError(t, err)
		assert.Equal(t, "ctx3", context)

		err = dbConn.QueryRow(ctx, `SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'mem1'`).Scan(&context)
		assert.NoError(t, err)
		assert.Equal(t, "ctx1_updated", context)
	})
}

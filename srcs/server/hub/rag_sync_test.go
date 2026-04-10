package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

func TestRAGSyncProvider_SQLite(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	defer sqliteDB.Close()

	provider := db.NewSqliteProvider(sqliteDB)
	ctx := context.Background()

	// Create tables manually for testing since sqlite in-memory does not run migrations
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	require.NoError(t, err)

	svc := NewRAGSyncProvider(provider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{
			ID:      "mem1",
			Context: "Context 1",
			Vector:  []float32{0.1, 0.2, 0.3},
		},
		{
			ID:      "mem2",
			Context: "Context 2",
			Vector:  []float32{0.4, 0.5, 0.6},
		},
	}
	err = svc.ProcessIncomingSync(ctx, records)
	require.NoError(t, err)

	// Since ProcessIncomingSync marks them as "synced", FetchPendingSyncs should return 0
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 0)

	// Insert a pending record
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, 'pending')`, "mem3", "Context 3")
	require.NoError(t, err)

	// Test FetchPendingSyncs
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 1)
	assert.Equal(t, "mem3", pending[0].ID)

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem3"})
	require.NoError(t, err)

	// Verify no pending syncs
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 0)
}

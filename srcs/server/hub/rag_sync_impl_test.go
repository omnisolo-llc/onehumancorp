package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDefaultRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Use in-memory SQLite for testing
	sqlDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Create table
	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	require.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES
			('1', 'test1', X'010203', 'pending'),
			('2', 'test2', X'040506', 'pending'),
			('3', 'test3', X'070809', 'synced');
	`)
	require.NoError(t, err)

	service := NewDefaultRAGSyncService(provider)

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		records, err := service.FetchPendingSyncs(ctx, 10)
		require.NoError(t, err)
		assert.Len(t, records, 2)
		assert.Equal(t, "1", records[0].ID)
		assert.Equal(t, "2", records[1].ID)
		assert.Equal(t, []byte{1, 2, 3}, records[0].Vector)
	})

	t.Run("FetchPendingSyncs_Limit", func(t *testing.T) {
		records, err := service.FetchPendingSyncs(ctx, 1)
		require.NoError(t, err)
		assert.Len(t, records, 1)
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := service.MarkSynced(ctx, []string{"1"})
		require.NoError(t, err)

		records, err := service.FetchPendingSyncs(ctx, 10)
		require.NoError(t, err)
		assert.Len(t, records, 1)
		assert.Equal(t, "2", records[0].ID)

		// Verify last_sync_at was updated
		var lastSyncAt sql.NullTime
		err = provider.QueryRow(ctx, "SELECT last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&lastSyncAt)
		require.NoError(t, err)
		assert.True(t, lastSyncAt.Valid)
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		incoming := []RAGSyncRecord{
			{ID: "4", Context: "test4", Vector: []byte{10, 11, 12}},
		}
		err := service.ProcessIncomingSync(ctx, incoming)
		require.NoError(t, err)

		var count int
		err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = '4' AND sync_status = 'synced'").Scan(&count)
		require.NoError(t, err)
		assert.Equal(t, 1, count)
	})
}

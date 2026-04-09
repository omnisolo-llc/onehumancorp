package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)
	defer sqliteDB.Close()

	provider := db.NewSqliteProvider(sqliteDB)
	defer provider.Close()

	service := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'memory 1', 'pending'), ('2', 'memory 2', 'pending')
	`)
	require.NoError(t, err)

	pending, err := service.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 2)

	err = service.MarkSynced(ctx, []string{"1"})
	require.NoError(t, err)

	pending2, err := service.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending2, 1)
	assert.Equal(t, "2", pending2[0].ID)

	incoming := []RAGSyncRecord{
		{
			ID:         "3",
			Context:    "memory 3",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	require.NoError(t, err)

	row := provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '3'")
	var content string
	err = row.Scan(&content)
	require.NoError(t, err)
	assert.Equal(t, "memory 3", content)
}

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

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	_, err = sqlDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return db.NewSqliteProvider(sqlDB)
}

func TestRAGSyncImpl_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test records
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')")
	require.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test2', 'synced')")
	require.NoError(t, err)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	require.Len(t, records, 1)
	assert.Equal(t, "1", records[0].ID)
	assert.Equal(t, SyncStatusPending, records[0].SyncStatus)
}

func TestRAGSyncImpl_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')")
	require.NoError(t, err)

	err = svc.MarkSynced(ctx, []string{"1"})
	require.NoError(t, err)

	row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'")
	var status string
	err = row.Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "synced", status)
}

func TestRAGSyncImpl_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	now := time.Now()
	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "cloud context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	require.NoError(t, err)

	row := provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '1'")
	var content, status string
	err = row.Scan(&content, &status)
	require.NoError(t, err)
	assert.Equal(t, "cloud context", content)
	assert.Equal(t, "synced", status)
}

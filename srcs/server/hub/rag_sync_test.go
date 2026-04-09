package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/stretchr/testify/require"
	_ "modernc.org/sqlite"
)

func setupDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`)
	require.NoError(t, err)
	return db
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	db := setupDB(t)
	defer db.Close()
	svc := hub.NewRAGSyncService(db)

	ctx := context.Background()

	_, err := db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')")
	require.NoError(t, err)
	_, err = db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test2', 'synced')")
	require.NoError(t, err)
	_, err = db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('3', 'test3', 'pending')")
	require.NoError(t, err)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	require.Len(t, records, 2)
	require.Equal(t, "1", records[0].ID)
	require.Equal(t, "3", records[1].ID)

	records, err = svc.FetchPendingSyncs(ctx, 1)
	require.NoError(t, err)
	require.Len(t, records, 1)
	require.Equal(t, "1", records[0].ID)
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	db := setupDB(t)
	defer db.Close()
	svc := hub.NewRAGSyncService(db)

	ctx := context.Background()

	_, err := db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')")
	require.NoError(t, err)

	err = svc.MarkSynced(ctx, []string{"1"})
	require.NoError(t, err)

	var status string
	var lastSync sql.NullTime
	err = db.QueryRow("SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&status, &lastSync)
	require.NoError(t, err)
	require.Equal(t, string(hub.SyncStatusSynced), status)
	require.True(t, lastSync.Valid)
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	db := setupDB(t)
	defer db.Close()
	svc := hub.NewRAGSyncService(db)

	ctx := context.Background()

	records := []hub.RAGSyncRecord{
		{ID: "100", Context: "incoming1", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
		{ID: "101", Context: "incoming2", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	require.NoError(t, err)

	var count int
	err = db.QueryRow("SELECT count(*) FROM autodream_memories").Scan(&count)
	require.NoError(t, err)
	require.Equal(t, 2, count)

	var status string
	var content string
	err = db.QueryRow("SELECT sync_status, content FROM autodream_memories WHERE id = '100'").Scan(&status, &content)
	require.NoError(t, err)
	require.Equal(t, string(hub.SyncStatusSynced), status)
	require.Equal(t, "incoming1", content)

	// Test upsert
	records = []hub.RAGSyncRecord{
		{ID: "100", Context: "incoming1_updated", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = svc.ProcessIncomingSync(ctx, records)
	require.NoError(t, err)

	err = db.QueryRow("SELECT sync_status, content FROM autodream_memories WHERE id = '100'").Scan(&status, &content)
	require.NoError(t, err)
	require.Equal(t, string(hub.SyncStatusSynced), status)
	require.Equal(t, "incoming1_updated", content)
}

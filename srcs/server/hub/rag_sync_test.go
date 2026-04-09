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

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	require.NoError(t, err)

	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			source_type TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME NULL
		);
	`)
	require.NoError(t, err)
	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncServiceImpl_SQLite(t *testing.T) {
	d := setupTestDB(t)
	svc := NewRAGSyncService(d)
	ctx := context.Background()

	// Insert test data directly
	_, err := d.Exec(ctx, `
		INSERT INTO autodream_memories (id, organization_id, agent_id, content, source_type, sync_status)
		VALUES ('test_1', 'org1', 'agent1', 'test content 1', 'test', 'pending')
	`)
	require.NoError(t, err)

	_, err = d.Exec(ctx, `
		INSERT INTO autodream_memories (id, organization_id, agent_id, content, source_type, sync_status)
		VALUES ('test_2', 'org1', 'agent1', 'test content 2', 'test', 'pending')
	`)
	require.NoError(t, err)

	_, err = d.Exec(ctx, `
		INSERT INTO autodream_memories (id, organization_id, agent_id, content, source_type, sync_status)
		VALUES ('test_3', 'org1', 'agent1', 'test content 3', 'test', 'synced')
	`)
	require.NoError(t, err)

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 2)

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"test_1"})
	require.NoError(t, err)

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, pending, 1)
	assert.Equal(t, "test_2", pending[0].ID)

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "test_4", Context: "new content", SyncStatus: SyncStatusSynced},
		{ID: "test_2", Context: "updated content", SyncStatus: SyncStatusSynced}, // Conflict update
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	require.NoError(t, err)

	// Verify incoming processing
	var content string
	var status string
	err = d.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'test_4'").Scan(&content, &status)
	require.NoError(t, err)
	assert.Equal(t, "new content", content)
	assert.Equal(t, "synced", status)

	err = d.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'test_2'").Scan(&content, &status)
	require.NoError(t, err)
	assert.Equal(t, "updated content", content)
	assert.Equal(t, "synced", status)
}

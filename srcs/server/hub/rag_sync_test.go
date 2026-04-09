package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", "file::memory:")
	require.NoError(t, err)

	provider := db.NewSqliteProvider(sqlDB)

	// Create autodream_memories table
	query := `CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	);`
	_, err = provider.Exec(context.Background(), query)
	require.NoError(t, err)

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, agent_id, source_type, content, sync_status) VALUES ('id1', 'org1', 'agent1', 'src', 'content 1', 'pending')")
	require.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, agent_id, source_type, content, sync_status) VALUES ('id2', 'org1', 'agent1', 'src', 'content 2', 'synced')")
	require.NoError(t, err)
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, agent_id, source_type, content, sync_status) VALUES ('id3', 'org1', 'agent1', 'src', 'content 3', 'pending')")
	require.NoError(t, err)

	service, err := NewRAGSyncService(provider)
	require.NoError(t, err)

	// Fetch with limit 1
	records, err := service.FetchPendingSyncs(ctx, 1)
	require.NoError(t, err)
	assert.Len(t, records, 1)
	assert.Equal(t, SyncStatusPending, records[0].SyncStatus)

	// Fetch with limit 10
	records, err = service.FetchPendingSyncs(ctx, 10)
	require.NoError(t, err)
	assert.Len(t, records, 2)
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, agent_id, source_type, content, sync_status) VALUES ('id1', 'org1', 'agent1', 'src', 'content 1', 'pending')")
	require.NoError(t, err)

	service, err := NewRAGSyncService(provider)
	require.NoError(t, err)

	err = service.MarkSynced(ctx, []string{"id1"})
	require.NoError(t, err)

	// Verify update
	rows, err := provider.Query(ctx, "SELECT sync_status FROM autodream_memories WHERE id = 'id1'")
	require.NoError(t, err)
	defer rows.Close()

	require.True(t, rows.Next())
	var status string
	err = rows.Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "synced", status)
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	ctx := context.Background()

	service, err := NewRAGSyncService(provider)
	require.NoError(t, err)

	records := []RAGSyncRecord{
		{
			ID:      "cloud-id-1",
			Context: "cloud content 1",
		},
		{
			ID:      "cloud-id-2",
			Context: "cloud content 2",
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	require.NoError(t, err)

	// Verify insertion
	rows, err := provider.Query(ctx, "SELECT id, content, sync_status FROM autodream_memories")
	require.NoError(t, err)
	defer rows.Close()

	var fetchedRecords []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus)
		require.NoError(t, err)
		fetchedRecords = append(fetchedRecords, r)
	}

	assert.Len(t, fetchedRecords, 2)
	assert.Equal(t, "cloud-id-1", fetchedRecords[0].ID)
	assert.Equal(t, "synced", string(fetchedRecords[0].SyncStatus))

	// Test upsert conflict
	updatedRecords := []RAGSyncRecord{
		{
			ID:      "cloud-id-1",
			Context: "updated cloud content 1",
		},
	}
	err = service.ProcessIncomingSync(ctx, updatedRecords)
	require.NoError(t, err)

	rows2, err := provider.Query(ctx, "SELECT content FROM autodream_memories WHERE id = 'cloud-id-1'")
	require.NoError(t, err)
	defer rows2.Close()

	require.True(t, rows2.Next())
	var updatedContent string
	err = rows2.Scan(&updatedContent)
	require.NoError(t, err)
	assert.Equal(t, "updated cloud content 1", updatedContent)
}

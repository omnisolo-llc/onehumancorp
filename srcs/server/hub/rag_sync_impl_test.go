package hub

import (
	"context"

	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	svc := NewRAGSyncService(provider)

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			organization_id  TEXT DEFAULT 'system',
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`)
	assert.NoError(t, err)

	// Clean up table if needed (test db is isolated, but good practice)
	provider.Exec(ctx, "DELETE FROM swarm_memory_embeddings")

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES
			('id1', 'ctx1', 'pending'),
			('id2', 'ctx2', 'synced'),
			('id3', 'ctx3', 'pending')
	`)
	assert.NoError(t, err)

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, records, 2)
	assert.Equal(t, "id1", records[0].ID)
	assert.Equal(t, "id3", records[1].ID)

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"id1"})
	assert.NoError(t, err)

	records, err = svc.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, records, 1)
	assert.Equal(t, "id3", records[0].ID)

	// Verify the database state
	var status string
	var lastSyncAt interface{}
	err = provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = 'id1'").Scan(&status, &lastSyncAt)
	assert.NoError(t, err)
	assert.Equal(t, "synced", status)
	assert.NotNil(t, lastSyncAt)

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "id4", Context: "ctx4"},
		{ID: "id1", Context: "ctx1_updated"},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	assert.NoError(t, err)

	var ctxStr string
	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'id1'").Scan(&ctxStr)
	assert.NoError(t, err)
	assert.Equal(t, "ctx1_updated", ctxStr)

	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'id4'").Scan(&ctxStr)
	assert.NoError(t, err)
	assert.Equal(t, "ctx4", ctxStr)
}

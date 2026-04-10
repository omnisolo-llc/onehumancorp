package hybrid_rag

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/stretchr/testify/assert"
)

func TestRAGSyncManager(t *testing.T) {
	ctx := context.Background()
	// Use New provider with :memory:
	t.Setenv("DATABASE_URL", "sqlite://:memory:")

	dbWrapper, err := db.New(ctx)
	assert.NoError(t, err)
	defer dbWrapper.Close()

	// Manually create the table and columns since migrations are failing due to missing base tables/order in this environment's go test
	_, err = dbWrapper.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP
		)
	`)
	assert.NoError(t, err)

	manager := NewRAGSyncManager(dbWrapper)

	// Seed data
	_, err = dbWrapper.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('test-1', 'context-1', 'pending')")
	assert.NoError(t, err)

	// Test FetchPendingSyncs
	records, err := manager.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, records, 1)
	assert.Equal(t, "test-1", records[0].ID)

	// Test MarkSynced
	err = manager.MarkSynced(ctx, []string{"test-1"})
	assert.NoError(t, err)

	// Verify marked as synced
	records, err = manager.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, records, 0)

	// Test ProcessIncomingSync
	incoming := []hub.RAGSyncRecord{
		{ID: "test-2", Context: "context-2"},
	}
	err = manager.ProcessIncomingSync(ctx, incoming)
	assert.NoError(t, err)

	// Verify incoming record exists
	var count int
	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = 'test-2'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)
}

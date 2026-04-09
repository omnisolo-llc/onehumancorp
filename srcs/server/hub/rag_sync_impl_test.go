package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

func createTestTable(ctx context.Context, dbProvider db.Provider) error {
	tx, err := dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	dbProvider := db.NewTestProvider(t)
	defer dbProvider.Close()

	assert.NoError(t, createTestTable(ctx, dbProvider))

	// Insert some mock data
	tx, _ := dbProvider.Begin(ctx)
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('2', 'ctx2', 'synced')")
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('3', 'ctx3', 'pending')")
	tx.Commit(ctx)

	svc := NewRAGSyncService(dbProvider)
	records, err := svc.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Len(t, records, 2)
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	ctx := context.Background()
	dbProvider := db.NewTestProvider(t)
	defer dbProvider.Close()

	assert.NoError(t, createTestTable(ctx, dbProvider))

	tx, _ := dbProvider.Begin(ctx)
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
	tx.Commit(ctx)

	svc := NewRAGSyncService(dbProvider)
	err := svc.MarkSynced(ctx, []string{"1"})
	assert.NoError(t, err)

	var status string
	var lastSyncAt *time.Time
	err = dbProvider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status, &lastSyncAt)
	assert.NoError(t, err)
	assert.Equal(t, "synced", status)
	assert.NotNil(t, lastSyncAt)
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	dbProvider := db.NewTestProvider(t)
	defer dbProvider.Close()

	assert.NoError(t, createTestTable(ctx, dbProvider))

	tx, _ := dbProvider.Begin(ctx)
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'old_ctx', 'synced')")
	tx.Commit(ctx)

	svc := NewRAGSyncService(dbProvider)
	records := []RAGSyncRecord{
		{ID: "1", Context: "new_ctx"},
		{ID: "2", Context: "ctx2"},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	assert.NoError(t, err)

	var ctx1, ctx2 string
	dbProvider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&ctx1)
	dbProvider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '2'").Scan(&ctx2)

	assert.Equal(t, "new_ctx", ctx1)
	assert.Equal(t, "ctx2", ctx2)
}

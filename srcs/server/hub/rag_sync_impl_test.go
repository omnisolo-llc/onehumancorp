package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/onehumancorp/mono/srcs/server/dbtest"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

func TestDefaultRAGSyncService(t *testing.T) {
	// Skip tests if we cannot instantiate test db easily, or use standard mock for db provider
	// We will create a quick test using an in-memory sqlite via db package if it supports it,
	// otherwise we mock db.Provider.

	// Mock implementation of db.Provider
	ctx := context.Background()
	provider := dbtest.NewTestProvider(t)
	defer provider.Close()

	// Initialize tables for test
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	assert.NoError(t, err)

	svc := hub.NewRAGSyncService(provider)

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		// Insert a pending record
		_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content', 'pending')`)
		assert.NoError(t, err)

		records, err := svc.FetchPendingSyncs(ctx, 10)
		assert.NoError(t, err)
		assert.Len(t, records, 1)
		assert.Equal(t, "1", records[0].ID)
		assert.Equal(t, "test content", records[0].Context)
		assert.Equal(t, hub.SyncStatusPending, records[0].SyncStatus)
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := svc.MarkSynced(ctx, []string{"1"})
		assert.NoError(t, err)

		// Verify sync_status is synced
		var status string
		err = provider.QueryRow(ctx, `SELECT sync_status FROM autodream_memories WHERE id = '1'`).Scan(&status)
		assert.NoError(t, err)
		assert.Equal(t, string(hub.SyncStatusSynced), status)
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		records := []hub.RAGSyncRecord{
			{
				ID:         "2",
				Context:    "cloud content",
				SyncStatus: hub.SyncStatusSynced,
				LastSyncAt: time.Now(),
			},
		}

		err := svc.ProcessIncomingSync(ctx, records)
		assert.NoError(t, err)

		// Verify inserted
		var content string
		err = provider.QueryRow(ctx, `SELECT content FROM autodream_memories WHERE id = '2'`).Scan(&content)
		assert.NoError(t, err)
		assert.Equal(t, "cloud content", content)
	})
}

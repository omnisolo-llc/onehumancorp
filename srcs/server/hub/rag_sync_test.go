package hub

import (
    "context"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
)

func TestRAGSyncService(t *testing.T) {
    t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
    ctx := context.Background()

    pool, err := db.New(ctx)
    require.NoError(t, err)
    defer pool.Close()

    err = pool.RunMigrations(ctx)
    require.NoError(t, err)

    service := NewRAGSyncService(pool)

    // Insert a dummy record using standard DB exec
    _, err = pool.Exec(ctx, "INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ($1, $2, $3, $4, $5)", "mem1", "org1", "test context", "agent", "pending")
    require.NoError(t, err)

    records, err := service.FetchPendingSyncs(ctx, 10)
    require.NoError(t, err)
    assert.Len(t, records, 1)
    assert.Equal(t, "mem1", records[0].ID)

    err = service.MarkSynced(ctx, []string{"mem1"})
    require.NoError(t, err)

    records2, err := service.FetchPendingSyncs(ctx, 10)
    require.NoError(t, err)
    assert.Len(t, records2, 0)

    now := time.Now()
    incoming := []RAGSyncRecord{
        {ID: "mem2", Context: "new context", SyncStatus: SyncStatusSynced, LastSyncAt: now},
    }
    err = service.ProcessIncomingSync(ctx, incoming)
    require.NoError(t, err)
}

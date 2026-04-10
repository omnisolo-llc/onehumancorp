package hub

import (
    "context"
    "database/sql"
    "testing"

    _ "modernc.org/sqlite"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
)

func setupTestDB(t *testing.T) db.Provider {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    require.NoError(t, err)

    _, err = sqlDB.Exec(`
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding TEXT,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at DATETIME NULL
        )
    `)
    require.NoError(t, err)

    return db.NewSqliteProvider(sqlDB)
}

func TestRAGSyncManager(t *testing.T) {
    provider := setupTestDB(t)
    manager := NewRAGSyncManager(provider)
    ctx := context.Background()

    _, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context', 'pending')`)
    require.NoError(t, err)

    records, err := manager.FetchPendingSyncs(ctx, 10)
    require.NoError(t, err)
    assert.Len(t, records, 1)
    assert.Equal(t, "1", records[0].ID)
    assert.Equal(t, SyncStatusPending, records[0].SyncStatus)

    err = manager.MarkSynced(ctx, []string{"1"})
    require.NoError(t, err)

    records, err = manager.FetchPendingSyncs(ctx, 10)
    require.NoError(t, err)
    assert.Len(t, records, 0)

    incoming := []RAGSyncRecord{
        {ID: "2", Context: "incoming test", Vector: []float32{0.1, 0.2}},
    }
    err = manager.ProcessIncomingSync(ctx, incoming)
    require.NoError(t, err)
}

package hub

import (
	"context"
	"testing"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/stretchr/testify/require"
    _ "modernc.org/sqlite"
    "database/sql"
)

func setupTestDB(t *testing.T) db.Provider {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    require.NoError(t, err)

    provider := db.NewSqliteProvider(sqlDB)

    _, err = provider.Exec(context.Background(), `
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT,
            sync_status TEXT,
            last_sync_at TIMESTAMP
        )
    `)
    require.NoError(t, err)

    return provider
}

func TestDefaultRAGSyncService(t *testing.T) {
    provider := setupTestDB(t)
    svc := NewDefaultRAGSyncService(provider)
    ctx := context.Background()

    // 1. Insert pending records
    _, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'ctx1', 'pending')")
    require.NoError(t, err)

    // 2. Fetch pending
    records, err := svc.FetchPendingSyncs(ctx, 10)
    require.NoError(t, err)
    require.Len(t, records, 1)
    require.Equal(t, "1", records[0].ID)

    // 3. Mark synced
    err = svc.MarkSynced(ctx, []string{"1"})
    require.NoError(t, err)

    // Verify marked
    var status string
    err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
    require.NoError(t, err)
    require.Equal(t, "synced", status)

    // 4. Process incoming sync (update)
    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "1", Context: "updated ctx1"},
    })
    require.NoError(t, err)

    var content string
    err = provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '1'").Scan(&content)
    require.NoError(t, err)
    require.Equal(t, "updated ctx1", content)

    // 5. Process incoming sync (insert)
    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "2", Context: "new ctx2"},
    })
    require.NoError(t, err)

    err = provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '2'").Scan(&content)
    require.NoError(t, err)
    require.Equal(t, "new ctx2", content)
}

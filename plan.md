1. **Mark Mission as In Progress**: Update the mission file with specific bash commands and verify.
   ```bash
   sed -i 's/status: PENDING/status: IN_PROGRESS/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   sed -i 's/agent: Researcher/agent: Taskmaster/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   head -n 10 .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   ```

2. **Create Database Migration**: Create `srcs/server/db/migrations/032_hybrid_rag_sync.sql` using specific SQLite-compatible syntax and verify.
   ```bash
   cat << 'INNER_EOF' > srcs/server/db/migrations/032_hybrid_rag_sync.sql
-- 032_hybrid_rag_sync.sql
-- Add sync_status and last_sync_at to swarm_memory_embeddings for Hybrid MCP RAG Protocol

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
INNER_EOF
   cat srcs/server/db/migrations/032_hybrid_rag_sync.sql
   ```

3. **Implement Go Interface**: Create `srcs/server/hub/rag_sync.go` with complete code and verify. The service connects to the database utilizing `db.Provider`.
   ```bash
   mkdir -p srcs/server/hub
   cat << 'INNER_EOF' > srcs/server/hub/rag_sync.go
package hub

import (
    "context"
    "database/sql"
    "fmt"
    "time"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    MemoryID         string
    Context          string
    VectorEmbedding  []byte
    SourcePlugin     sql.NullString
    SyncStatus       SyncStatus
    LastSyncAt       sql.NullTime
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    ragRecordsSyncedTotal, _ = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total RAG records synced"),
    )
    ragSyncErrorsTotal, _ = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total RAG sync errors"),
    )
)

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := \`
        SELECT memory_id, context, vector_embedding, source_plugin, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        WHERE sync_status = 'pending'
        LIMIT $1
    \`

    rows, err := s.db.Query(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var syncStatus sql.NullString

        err := rows.Scan(
            &record.MemoryID,
            &record.Context,
            &record.VectorEmbedding,
            &record.SourcePlugin,
            &syncStatus,
            &record.LastSyncAt,
        )
        if err != nil {
            return nil, fmt.Errorf("failed to scan pending sync: %w", err)
        }

        if syncStatus.Valid {
            record.SyncStatus = SyncStatus(syncStatus.String)
        } else {
            record.SyncStatus = SyncStatusPending
        }

        records = append(records, record)
    }

    if err := rows.Err(); err != nil {
        return nil, fmt.Errorf("error iterating pending syncs: %w", err)
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, id := range ids {
        query := \`
            UPDATE swarm_memory_embeddings
            SET sync_status = 'synced', last_sync_at = $1
            WHERE memory_id = $2
        \`
        _, err := tx.Exec(ctx, query, now, id)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, record := range records {
        query := \`
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, 'synced', $5)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                source_plugin = EXCLUDED.source_plugin,
                sync_status = 'synced',
                last_sync_at = EXCLUDED.last_sync_at
        \`

        _, err := tx.Exec(ctx, query,
            record.MemoryID,
            record.Context,
            record.VectorEmbedding,
            record.SourcePlugin,
            record.LastSyncAt,
        )
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to process incoming sync for memory_id %s: %w", record.MemoryID, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    return nil
}
INNER_EOF
   cat srcs/server/hub/rag_sync.go
   ```

4. **Implement Tests**: Create `srcs/server/hub/rag_sync_test.go` and verify.
   ```bash
   cat << 'INNER_EOF' > srcs/server/hub/rag_sync_test.go
package hub

import (
    "context"
    "database/sql"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
    t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
    ctx := context.Background()

    database, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to create db: %v", err)
    }
    defer database.Close()

    _, err = database.Exec(ctx, \`
        CREATE TABLE swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     TIMESTAMP NULL
        )
    \`)
    if err != nil {
        t.Fatalf("failed to create schema: %v", err)
    }

    service := NewRAGSyncService(database.Provider)

    // Setup initial data
    _, err = database.Exec(ctx, \`
        INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
        VALUES ('mem1', 'ctx1', 'pending')
    \`)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].MemoryID != "mem1" {
        t.Errorf("expected MemoryID 'mem1', got '%s'", records[0].MemoryID)
    }

    err = service.MarkSynced(ctx, []string{"mem1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    records, err = service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
    }

    // Process incoming sync
    incoming := []RAGSyncRecord{
        {
            MemoryID: "mem2",
            Context: "ctx2",
            VectorEmbedding: []byte("vec2"),
            SourcePlugin: sql.NullString{String: "plugin1", Valid: true},
            SyncStatus: SyncStatusSynced,
            LastSyncAt: sql.NullTime{Time: time.Now(), Valid: true},
        },
    }
    err = service.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("expected no error processing incoming sync, got %v", err)
    }

    var count int
    row := database.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = 'mem2'")
    err = row.Scan(&count)
    if err != nil || count != 1 {
        t.Fatalf("expected 1 record for mem2, got %d, err: %v", count, err)
    }
}
INNER_EOF
   cat srcs/server/hub/rag_sync_test.go
   ```

5. **Update BUILD.bazel**: Generate and adjust BUILD.bazel for the new code.
   ```bash
   export PATH="$PATH:$HOME/go/bin"
   bazelisk run //:gazelle
   cat srcs/server/hub/BUILD.bazel
   ```

6. **Mark Mission as Done**: Update mission file status to `DONE` and verify.
   ```bash
   sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   head -n 10 .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
   ```

7. **Test and Verify Code**: Ensure tests pass.
   ```bash
   export PATH="$PATH:$HOME/go/bin"
   bazelisk test //srcs/server/hub/...
   ```

8. **Pre-commit Checks**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit PR**: Submit the final changes.

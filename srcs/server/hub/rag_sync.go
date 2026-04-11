package hub

import (
    "context"
    "time"
    "encoding/json"
    "fmt"
    "database/sql"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
    "github.com/onehumancorp/mono/srcs/server/db"
)

var (
	meter         = otel.Meter("hub_rag_sync")
	syncedRecords metric.Int64Counter
	syncErrors    metric.Int64Counter
)

func init() {
	var err error
	syncedRecords, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		panic(err)
	}
	syncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		panic(err)
	}
}

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID           string
    Context      string
    Vector       []float32 // Convert to string internally for SQLite compat if needed
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    // FetchPendingSyncs retrieves records from the local DB that need syncing
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

    // MarkSynced updates the local DB after a successful sync to the cloud
    MarkSynced(ctx context.Context, ids []string) error

    // ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, embedding, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = $1
        LIMIT $2
    `
    rows, err := s.db.Query(ctx, query, SyncStatusPending, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt sql.NullTime
        var embeddingStr sql.NullString
        var vector []float32

        err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &rec.SyncStatus, &lastSyncAt)
        if err != nil {
            return nil, err
        }
        if lastSyncAt.Valid {
            rec.LastSyncAt = lastSyncAt.Time
        }

        if embeddingStr.Valid {
            err = json.Unmarshal([]byte(embeddingStr.String), &vector)
            if err == nil {
               rec.Vector = vector
            }
        }

        records = append(records, rec)
    }

    if err = rows.Err(); err != nil {
        return nil, err
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    query := `
        UPDATE autodream_memories
        SET sync_status = $1, last_sync_at = $2
        WHERE id = $3
    `
    now := time.Now()
    for _, id := range ids {
        _, err := tx.Exec(ctx, query, SyncStatusSynced, now, id)
        if err != nil {
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return err
    }

    syncedRecords.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    var query string
    if s.db.IsSQLite() {
        query = `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
            VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
            ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                embedding = CAST(excluded.embedding AS TEXT),
                sync_status = excluded.sync_status,
                last_sync_at = excluded.last_sync_at
        `
    } else {
        query = `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3::vector, $4, $5)
            ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                embedding = excluded.embedding,
                sync_status = excluded.sync_status,
                last_sync_at = excluded.last_sync_at
        `
    }

    for _, rec := range records {
        vectorBytes, err := json.Marshal(rec.Vector)
        if err != nil {
             return fmt.Errorf("failed to marshal vector: %w", err)
        }

        _, err = tx.Exec(ctx, query, rec.ID, rec.Context, string(vectorBytes), SyncStatusSynced, time.Now())
        if err != nil {
            syncErrors.Add(ctx, 1)
            return err
        }
    }

    return tx.Commit(ctx)
}

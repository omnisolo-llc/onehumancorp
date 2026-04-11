package hub

import (
    "context"
    "time"
    "encoding/json"
    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
)

type SyncStatus string

const (
    SyncStatusPending    SyncStatus = "pending"
    SyncStatusInProgress SyncStatus = "in_progress"
    SyncStatusSynced     SyncStatus = "synced"
    SyncStatusError      SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID                string
    Context           string
    Vector            []float32
    SyncStatus        SyncStatus
    LastSyncTimestamp time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type syncService struct {
    dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &syncService{dbProvider: provider}
}

var meter = otel.GetMeterProvider().Meter("hub/rag_sync")
var (
    RecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
    SyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total")
)

func (s *syncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    defer tx.Rollback(ctx)

    query := "SELECT id, content, embedding, sync_status, last_sync_timestamp FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
    if !s.dbProvider.IsSQLite() {
        query += " FOR UPDATE SKIP LOCKED"
    }

    rows, err := tx.Query(ctx, query, limit)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }

    var records []RAGSyncRecord
    var ids []string
    for rows.Next() {
        var r RAGSyncRecord
        var vecStr string
        var lastSyncAt *time.Time
        if err := rows.Scan(&r.ID, &r.Context, &vecStr, &r.SyncStatus, &lastSyncAt); err != nil {
            continue
        }
        if lastSyncAt != nil {
            r.LastSyncTimestamp = *lastSyncAt
        }
        json.Unmarshal([]byte(vecStr), &r.Vector)
        records = append(records, r)
        ids = append(ids, r.ID)
    }
    rows.Close()

    if len(ids) > 0 {
        // Transition to in_progress to avoid concurrent fetching
        for _, id := range ids {
            updateQuery := "UPDATE autodream_memories SET sync_status = 'in_progress' WHERE id = $1"
            _, err := tx.Exec(ctx, updateQuery, id)
            if err != nil {
                SyncErrorsTotal.Add(ctx, 1)
                return nil, err
            }
        }
    }

    if err := tx.Commit(ctx); err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }

    return records, nil
}

func (s *syncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    for _, id := range ids {
        query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE id = $1"
        _, err := s.dbProvider.Exec(ctx, query, id)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
        RecordsSyncedTotal.Add(ctx, 1)
    }
    return nil
}

func (s *syncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, r := range records {
        vecBytes, _ := json.Marshal(r.Vector)
        query := `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP`
        _, err := s.dbProvider.Exec(ctx, query, r.ID, r.Context, string(vecBytes))
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    return nil
}

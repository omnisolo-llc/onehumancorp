package hub

import (
	"context"
	"time"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
    provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
    return &DefaultRAGSyncService{
        provider: provider,
    }
}

func (s *DefaultRAGSyncService) isSQLite() bool {
    _, isSqlite := s.provider.(*db.SqliteProvider)
    return isSqlite
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
    if s.isSQLite() {
        query = "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?"
    }

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }

    return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    for _, id := range ids {
        query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
        if s.isSQLite() {
            query = "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
        }

        _, err := s.provider.Exec(ctx, query, id)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil { telemetry.RagSyncErrorsTotal.Add(ctx, 1) }
            return err
        }
        if telemetry.RagRecordsSyncedTotal != nil { telemetry.RagRecordsSyncedTotal.Add(ctx, 1) }
    }

    return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, rec := range records {
        query := "UPDATE autodream_memories SET content = $1, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $2"
        if s.isSQLite() {
            query = "UPDATE autodream_memories SET content = ?, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
        }

        rowsAffected, err := s.provider.Exec(ctx, query, rec.Context, rec.ID)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil { telemetry.RagSyncErrorsTotal.Add(ctx, 1) }
            return err
        }

        if err == nil && rowsAffected == 0 {
            // Upsert fallback
            insertQuery := "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)"
            if s.isSQLite() {
                insertQuery = "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)"
            }
            _, err = s.provider.Exec(ctx, insertQuery, rec.ID, rec.Context)
            if err != nil {
                if telemetry.RagSyncErrorsTotal != nil { telemetry.RagSyncErrorsTotal.Add(ctx, 1) }
                return err
            }
        }
        if telemetry.RagRecordsSyncedTotal != nil { telemetry.RagRecordsSyncedTotal.Add(ctx, 1) }
    }
    return nil
}

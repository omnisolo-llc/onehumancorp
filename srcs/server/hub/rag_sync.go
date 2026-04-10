package hub

import (
    "context"
    "time"
	"log/slog"

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

type RAGSyncServiceImpl struct {
    db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
    return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    rows, err := s.db.Query(ctx, query, limit)
    if err != nil {
        RecordRAGSyncError(ctx, 1)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        var syncStatus *string
        if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt); err != nil {
            RecordRAGSyncError(ctx, 1)
            return nil, err
        }
        if syncStatus != nil {
            rec.SyncStatus = SyncStatus(*syncStatus)
        } else {
            rec.SyncStatus = SyncStatusPending
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }
    return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Simplistic batch update for demonstration. In production, use unnest or a batched transaction
    tx, err := s.db.Begin(ctx)
    if err != nil {
        RecordRAGSyncError(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        _, err := tx.Exec(ctx, `
            UPDATE autodream_memories
            SET sync_status = 'synced', last_sync_at = $1
            WHERE id = $2
        `, time.Now(), id)
        if err != nil {
            RecordRAGSyncError(ctx, 1)
            return err
        }
    }

    err = tx.Commit(ctx)
    if err == nil {
        RecordRAGRecordSynced(ctx, int64(len(ids)))
    } else {
        RecordRAGSyncError(ctx, 1)
    }
    return err
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        RecordRAGSyncError(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        _, err := tx.Exec(ctx, `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
            VALUES ($1, $2, 'synced', $3)
            ON CONFLICT (id) DO UPDATE
            SET content = EXCLUDED.content,
                sync_status = 'synced',
                last_sync_at = EXCLUDED.last_sync_at
        `, rec.ID, rec.Context, time.Now())
        if err != nil {
            RecordRAGSyncError(ctx, 1)
            return err
        }
    }

    err = tx.Commit(ctx)
    if err != nil {
        RecordRAGSyncError(ctx, 1)
    }
    return err
}

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("ohc-hub")
	var err error

	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		slog.Error("failed to initialize rag_records_synced_total metric", "error", err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		slog.Error("failed to initialize rag_sync_errors_total metric", "error", err)
	}
}

// RecordRAGRecordSynced increments the counter for successfully synced RAG records.
func RecordRAGRecordSynced(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count)
	}
}

// RecordRAGSyncError increments the counter for RAG sync errors.
func RecordRAGSyncError(ctx context.Context, count int64) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, count)
	}
}

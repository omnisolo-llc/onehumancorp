package hub

import (
    "context"
    "time"
    "database/sql"
    "fmt"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
    "github.com/onehumancorp/mono/srcs/server/auth"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
    if err != nil {
        panic(err)
    }
    SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
    if err != nil {
        panic(err)
    }
}

// SQLRAGSyncService is a concrete implementation of RAGSyncService
type SQLRAGSyncService struct {
    db *sql.DB
}

// NewSQLRAGSyncService creates a new SQLRAGSyncService
func NewSQLRAGSyncService(db *sql.DB) *SQLRAGSyncService {
    return &SQLRAGSyncService{db: db}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    claims := auth.ClaimsFromContext(ctx)
    var organizationID string
    if claims != nil {
        organizationID = claims.OrganizationID
    }

    var query string
    var rows *sql.Rows
    var err error

    if organizationID != "" {
        query = `
            SELECT id, content, sync_status, last_sync_at
            FROM consolidated_memory
            WHERE sync_status = $1 AND organization_id = $2
            LIMIT $3
        `
        rows, err = s.db.QueryContext(ctx, query, SyncStatusPending, organizationID, limit)
    } else {
        query = `
            SELECT id, content, sync_status, last_sync_at
            FROM consolidated_memory
            WHERE sync_status = $1
            LIMIT $2
        `
        rows, err = s.db.QueryContext(ctx, query, SyncStatusPending, limit)
    }

    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt sql.NullTime
        if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
        }
        if lastSyncAt.Valid {
            r.LastSyncAt = lastSyncAt.Time
        }
        records = append(records, r)
    }

    if err := rows.Err(); err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, fmt.Errorf("error iterating pending syncs: %w", err)
    }

    return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    claims := auth.ClaimsFromContext(ctx)
    var organizationID string
    if claims != nil {
        organizationID = claims.OrganizationID
    }

    tx, err := s.db.BeginTx(ctx, nil)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback()

    var stmt *sql.Stmt
    if organizationID != "" {
        stmt, err = tx.PrepareContext(ctx, "UPDATE consolidated_memory SET sync_status = $1, last_sync_at = $2 WHERE id = $3 AND organization_id = $4")
    } else {
        stmt, err = tx.PrepareContext(ctx, "UPDATE consolidated_memory SET sync_status = $1, last_sync_at = $2 WHERE id = $3")
    }
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to prepare mark synced statement: %w", err)
    }
    defer stmt.Close()

    now := time.Now()
    for _, id := range ids {
        if organizationID != "" {
            _, err = stmt.ExecContext(ctx, SyncStatusSynced, now, id, organizationID)
        } else {
            _, err = stmt.ExecContext(ctx, SyncStatusSynced, now, id)
        }

        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
        }
    }

    if err := tx.Commit(); err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to commit mark synced transaction: %w", err)
    }

    RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    claims := auth.ClaimsFromContext(ctx)
    var organizationID string
    if claims != nil {
        organizationID = claims.OrganizationID
    }

    // Fallback if not authenticated properly
    if organizationID == "" {
        organizationID = "default_org"
    }

    tx, err := s.db.BeginTx(ctx, nil)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback()

    // Using an atomic UPSERT (ON CONFLICT) strategy
    stmt, err := tx.PrepareContext(ctx, `
        INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status, last_sync_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id) DO UPDATE SET
            content = EXCLUDED.content,
            sync_status = EXCLUDED.sync_status,
            last_sync_at = EXCLUDED.last_sync_at
        WHERE consolidated_memory.organization_id = EXCLUDED.organization_id
    `)

    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to prepare upsert statement: %w", err)
    }
    defer stmt.Close()

    now := time.Now()
    for _, r := range records {
        _, err := stmt.ExecContext(ctx, r.ID, organizationID, r.Context, "sync_import", SyncStatusSynced, now)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to upsert incoming sync record %s: %w", r.ID, err)
        }
    }

    if err := tx.Commit(); err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to commit process incoming sync transaction: %w", err)
    }

    RecordsSyncedTotal.Add(ctx, int64(len(records)))
    return nil
}

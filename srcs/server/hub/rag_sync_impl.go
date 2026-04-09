package hub

import (
    "context"
    "database/sql"
    "fmt"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type DatabaseRAGSyncService struct {
    db *sql.DB
}

func NewDatabaseRAGSyncService(db *sql.DB) *DatabaseRAGSyncService {
    return &DatabaseRAGSyncService{db: db}
}

func (s *DatabaseRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, sync_status, last_sync_at
        FROM consolidated_memory
        WHERE sync_status = 'pending' OR sync_status IS NULL
        LIMIT $1
    `
    rows, err := s.db.QueryContext(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt sql.NullTime
        var syncStatus sql.NullString

        if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt); err != nil {
            return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
        }

        if syncStatus.Valid {
            rec.SyncStatus = SyncStatus(syncStatus.String)
        } else {
            rec.SyncStatus = SyncStatusPending
        }

        if lastSyncAt.Valid {
            rec.LastSyncAt = lastSyncAt.Time
        }

        records = append(records, rec)
    }

    if err := rows.Err(); err != nil {
        return nil, fmt.Errorf("error iterating pending syncs: %w", err)
    }

    return records, nil
}

func (s *DatabaseRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Creating placeholders like $1, $2, $3...
    placeholders := make([]string, len(ids))
    args := make([]interface{}, len(ids))
    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+1)
        args[i] = id
    }

    query := fmt.Sprintf(`
        UPDATE consolidated_memory
        SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
        WHERE id IN (%s)
    `, strings.Join(placeholders, ","))

    result, err := s.db.ExecContext(ctx, query, args...)
    if err != nil {
        if telemetry.RagSyncErrorsTotal != nil {
            telemetry.RagSyncErrorsTotal.Add(ctx, 1)
        }
        return fmt.Errorf("failed to mark synced: %w", err)
    }

    rowsAffected, _ := result.RowsAffected()
    if telemetry.RagRecordsSyncedTotal != nil {
        telemetry.RagRecordsSyncedTotal.Add(ctx, rowsAffected)
    }

    return nil
}

func (s *DatabaseRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.db.BeginTx(ctx, nil)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback()

    // Check if it's Postgres to use native UPSERT ON CONFLICT or simple query for SQLite testing.
    // For universal compatibility across SQLite/Postgres without knowing the driver type,
    // we can attempt an update, and if no rows are affected, we do an insert.
    updateQuery := `
        UPDATE consolidated_memory
        SET content = $1, sync_status = $2, last_sync_at = $3
        WHERE id = $4
    `
    updateStmt, err := tx.PrepareContext(ctx, updateQuery)
    if err != nil {
        return fmt.Errorf("failed to prepare update statement: %w", err)
    }
    defer updateStmt.Close()

    insertQuery := `
        INSERT INTO consolidated_memory (id, content, sync_status, last_sync_at, organization_id, source_type)
        VALUES ($1, $2, $3, $4, 'default', 'sync')
    `
    insertStmt, err := tx.PrepareContext(ctx, insertQuery)
    if err != nil {
        return fmt.Errorf("failed to prepare insert statement: %w", err)
    }
    defer insertStmt.Close()

    for _, rec := range records {
        res, err := updateStmt.ExecContext(ctx, rec.Context, string(rec.SyncStatus), rec.LastSyncAt, rec.ID)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("failed to process incoming sync record update %s: %w", rec.ID, err)
        }

        rowsAffected, err := res.RowsAffected()
        if err != nil {
             return fmt.Errorf("failed to get rows affected: %w", err)
        }

        if rowsAffected == 0 {
            // Update didn't affect anything, meaning record doesn't exist. Insert it.
            _, err = insertStmt.ExecContext(ctx, rec.ID, rec.Context, string(rec.SyncStatus), rec.LastSyncAt)
            if err != nil {
                if telemetry.RagSyncErrorsTotal != nil {
                    telemetry.RagSyncErrorsTotal.Add(ctx, 1)
                }
                return fmt.Errorf("failed to process incoming sync record insert %s: %w", rec.ID, err)
            }
        }
    }

    if err := tx.Commit(); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    if telemetry.RagRecordsSyncedTotal != nil {
        telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
    }

    return nil
}

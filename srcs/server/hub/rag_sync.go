package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

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

type ragSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &ragSyncService{db: db}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		ORDER BY created_at ASC
		LIMIT $2
	`
	rows, err := s.db.QueryContext(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vectorStr sql.NullString
		var statusStr sql.NullString

		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &statusStr, &lastSyncAt); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		if statusStr.Valid {
			r.SyncStatus = SyncStatus(statusStr.String)
		} else {
			r.SyncStatus = SyncStatusPending
		}

		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}

		if vectorStr.Valid && vectorStr.String != "" {
			if err := json.Unmarshal([]byte(vectorStr.String), &r.Vector); err != nil {
				// Log error but continue, embedding might be invalid format
				telemetry.RecordRAGSyncError(ctx)
			}
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("error iterating pending sync rows: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	query := `
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id = $3
	`
	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to prepare update statement: %w", err)
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, string(SyncStatusSynced), now, id); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGRecordsSynced(ctx, len(ids))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	// Implement UPSERT logic using UPDATE then INSERT for SQLite/Postgres compatibility
	updateQuery := `
		UPDATE autodream_memories
		SET content = $1, embedding = $2, sync_status = $3, last_sync_at = $4
		WHERE id = $5
	`
	insertQuery := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
	`
	updateStmt, err := tx.PrepareContext(ctx, updateQuery)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to prepare update statement: %w", err)
	}
	defer updateStmt.Close()

	insertStmt, err := tx.PrepareContext(ctx, insertQuery)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to prepare insert statement: %w", err)
	}
	defer insertStmt.Close()

	now := time.Now()
	for _, r := range records {
		var vectorStr string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err == nil {
				vectorStr = string(b)
			}
		}

		res, err := updateStmt.ExecContext(ctx, r.Context, vectorStr, string(SyncStatusSynced), now, r.ID)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to execute update for record %s: %w", r.ID, err)
		}

		rowsAffected, err := res.RowsAffected()
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to get rows affected: %w", err)
		}

		if rowsAffected == 0 {
			if _, err := insertStmt.ExecContext(ctx, r.ID, r.Context, vectorStr, string(SyncStatusSynced), now); err != nil {
				telemetry.RecordRAGSyncError(ctx)
				return fmt.Errorf("failed to execute insert for record %s: %w", r.ID, err)
			}
		}
	}

	if err := tx.Commit(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

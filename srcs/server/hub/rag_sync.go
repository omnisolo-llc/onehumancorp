package hub

import (
	"context"
	"database/sql"
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
	Vector     []float32
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

type StandaloneRAGSyncService struct {
	db *sql.DB
}

func NewStandaloneRAGSyncService(db *sql.DB) *StandaloneRAGSyncService {
	return &StandaloneRAGSyncService{db: db}
}

func (s *StandaloneRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT ?
	`, limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		var syncStatus sql.NullString
		if err := rows.Scan(&r.ID, &r.Context, &syncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		if syncStatus.Valid && syncStatus.String != "" {
			r.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			r.SyncStatus = SyncStatusPending
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return records, err
	}
	return records, nil
}

func (s *StandaloneRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Basic batch update for sqlite
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id = ?
	`)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer stmt.Close()

	var syncedCount int64
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, id); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
		syncedCount++
	}

	if err := tx.Commit(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}

	telemetry.RecordRAGRecordsSynced(ctx, syncedCount)
	return nil
}

func (s *StandaloneRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES (?, ?, ?, ?)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
	`)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer stmt.Close()

	var importedCount int64
	for _, r := range records {
		var lastSyncAt interface{}
		if r.LastSyncAt.IsZero() {
			lastSyncAt = nil
		} else {
			lastSyncAt = r.LastSyncAt
		}
		if _, err := stmt.ExecContext(ctx, r.ID, r.Context, r.SyncStatus, lastSyncAt); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
		importedCount++
	}

	if err := tx.Commit(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}

	telemetry.RecordRAGRecordsSynced(ctx, importedCount)
	return nil
}

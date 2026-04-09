package hub

import (
	"context"
	"database/sql"
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

type DefaultRAGSyncService struct {
	db *sql.DB
}

func NewDefaultRAGSyncService(db *sql.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, fmt.Errorf("query failed: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("scan failed: %w", err)
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2")
	if err != nil {
		return err
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, now, id); err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
			return err
		}
		if telemetry.RAGRecordsSyncedTotal != nil {
			telemetry.RAGRecordsSyncedTotal.Add(ctx, 1)
		}
	}

	return tx.Commit()
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Example implementation (assuming vector logic exists elsewhere or is omitted for now)
	stmt, err := tx.PrepareContext(ctx, "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, 'synced', $3) ON CONFLICT(id) DO UPDATE SET content = excluded.content, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at")
	if err != nil {
		return err
	}
	defer stmt.Close()

	now := time.Now()
	for _, r := range records {
		if _, err := stmt.ExecContext(ctx, r.ID, r.Context, now); err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
			return err
		}
		if telemetry.RAGRecordsSyncedTotal != nil {
			telemetry.RAGRecordsSyncedTotal.Add(ctx, 1)
		}
	}

	return tx.Commit()
}

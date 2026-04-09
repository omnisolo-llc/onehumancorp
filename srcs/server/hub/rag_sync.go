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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type sqlRAGSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &sqlRAGSyncService{db: db}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var content sql.NullString
		var status sql.NullString
		if err := rows.Scan(&r.ID, &content, &status); err != nil {
			return nil, err
		}
		if content.Valid {
			r.Context = content.String
		}
		if status.Valid {
			r.SyncStatus = SyncStatus(status.String)
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()
	for _, id := range ids {
		_, err := s.db.ExecContext(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3", SyncStatusSynced, now, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return err
		}
		telemetry.RecordRAGSyncSuccess(ctx, 1)
	}
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, r := range records {
		_, err := tx.ExecContext(ctx, "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET content = excluded.content, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at", r.ID, r.Context, SyncStatusSynced, now)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return err
		}
		telemetry.RecordRAGSyncSuccess(ctx, 1)
	}
	return tx.Commit()
}

package hub

import (
	"context"
	"database/sql"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SQLRAGSyncService struct {
	db *sql.DB
}

func NewSQLRAGSyncService(db *sql.DB) *SQLRAGSyncService {
	return &SQLRAGSyncService{db: db}
}

func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, id := range ids {
		_, err := tx.ExecContext(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", now, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return err
		}
	}

	telemetry.RecordRAGSyncSuccess(ctx, len(ids))
	return tx.Commit()
}

func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, r := range records {
		var exists bool
		err := tx.QueryRowContext(ctx, "SELECT EXISTS(SELECT 1 FROM autodream_memories WHERE id = $1)", r.ID).Scan(&exists)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return err
		}

		if exists {
			_, err = tx.ExecContext(ctx, "UPDATE autodream_memories SET content = $1, embedding = $2, sync_status = 'synced', last_sync_at = $3 WHERE id = $4", r.Context, r.Vector, now, r.ID)
		} else {
			if r.ID == "" {
				r.ID = uuid.New().String()
			}
			_, err = tx.ExecContext(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4)", r.ID, r.Context, r.Vector, now)
		}

		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return err
		}
	}

	telemetry.RecordRAGSyncSuccess(ctx, len(records))
	return tx.Commit()
}

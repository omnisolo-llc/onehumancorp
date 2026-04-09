package hub

import (
	"context"
	"database/sql"
	"encoding/json"
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DBRAGSyncService struct {
	db *sql.DB
}

func NewDBRAGSyncService(db *sql.DB) *DBRAGSyncService {
	return &DBRAGSyncService{db: db}
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var content string
		var status string
		var embeddingStr sql.NullString

		if err := rows.Scan(&r.ID, &content, &embeddingStr, &status); err != nil {
			return nil, err
		}

		r.Context = content
		r.SyncStatus = SyncStatus(status)

		if embeddingStr.Valid {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingStr.String), &vec); err == nil {
				r.Vector = vec
			}
		}

		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1")
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer stmt.Close()

	for _, id := range ids {
		_, err := stmt.ExecContext(ctx, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}

	for range ids {
		telemetry.RecordRAGSyncSuccess(ctx)
	}
	return nil
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(id) DO UPDATE SET content = excluded.content, embedding = excluded.embedding, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at")
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	defer stmt.Close()

	for _, r := range records {
		var vecStr sql.NullString
		if len(r.Vector) > 0 {
			b, _ := json.Marshal(r.Vector)
			vecStr = sql.NullString{String: string(b), Valid: true}
		}

		_, err := stmt.ExecContext(ctx, r.ID, r.Context, vecStr, string(r.SyncStatus), time.Now())
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}

	for range records {
		telemetry.RecordRAGSyncSuccess(ctx)
	}
	return nil
}

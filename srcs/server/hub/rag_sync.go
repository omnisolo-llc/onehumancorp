package hub

import (
	"context"
	"time"
	"database/sql"
	"errors"
	"encoding/json"

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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
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

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Must cast embedding to text for both Postgres and SQLite compat during sync fetch
	query := "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	if s.provider.IsSQLite() {
		query = "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var emb *string
		if err := rows.Scan(&r.ID, &r.Context, &emb, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		if emb != nil {
			var vec []float32
			if err := json.Unmarshal([]byte(*emb), &vec); err == nil {
				r.Vector = vec
			}
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

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		if s.provider.IsSQLite() {
			query = "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
		}
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		telemetry.RecordRagRecordsSynced(ctx, len(ids))
	} else {
		telemetry.RecordRagSyncError(ctx)
	}
	return err
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var embBytes []byte
		var embStr interface{}
		if len(r.Vector) > 0 {
			embBytes, _ = json.Marshal(r.Vector)
			embStr = string(embBytes)
		} else {
			embStr = nil
		}

		query := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP"
		if s.provider.IsSQLite() {
			query = "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = excluded.content, embedding = excluded.embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP"
		}

		_, err := tx.Exec(ctx, query, r.ID, r.Context, embStr)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return err
	}
	return nil
}

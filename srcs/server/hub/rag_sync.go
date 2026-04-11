package hub

import (
	"context"
	"fmt"
	"time"

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
	Vector     []byte // Changed from []float32 to []byte for DB compatibility
	SyncStatus SyncStatus
	LastSyncAt *time.Time
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
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT "
	if s.provider.IsSQLite() {
		query += "?"
	} else {
		query += "$1"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		rec.LastSyncAt = lastSyncAt
		records = append(records, rec)
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

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = "
		if s.provider.IsSQLite() {
			query += "?"
		} else {
			query += "$1"
		}
		_, err := s.provider.Exec(ctx, query, id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}

	var syncedCount int64
	for _, rec := range records {
		var query string
		if s.provider.IsSQLite() {
			query = "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET content=excluded.content, embedding=excluded.embedding, sync_status=excluded.sync_status, last_sync_at=excluded.last_sync_at"
		} else {
			query = "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding, sync_status=EXCLUDED.sync_status, last_sync_at=EXCLUDED.last_sync_at"
		}
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, string(rec.SyncStatus), rec.LastSyncAt)
		if err != nil {
			tx.Rollback(ctx)
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to process incoming sync for %s: %w", rec.ID, err)
		}
		syncedCount++
	}

	err = tx.Commit(ctx)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGRecordsSynced(ctx, syncedCount)
	return nil
}

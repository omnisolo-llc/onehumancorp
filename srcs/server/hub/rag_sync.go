package hub

import (
	"context"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type sqlRAGSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &sqlRAGSyncService{provider: provider}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"

	// Support SQLite "?" placeholder
	if s.provider.IsSQLite() {
		query = "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
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
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}
	telemetry.RecordRAGRecordsSynced(ctx, len(ids))
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, record := range records {
		// Basic Last-Write-Wins (LWW) UPSERT logic
		query := `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		          VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		          ON CONFLICT(id) DO UPDATE SET content = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
		if s.provider.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			          VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)
			          ON CONFLICT(id) DO UPDATE SET content = ?, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
			_, err = tx.Exec(ctx, query, record.ID, record.Context, record.Context)
		} else {
			_, err = tx.Exec(ctx, query, record.ID, record.Context)
		}

		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}
	telemetry.RecordRAGRecordsSynced(ctx, len(records))
	return nil
}

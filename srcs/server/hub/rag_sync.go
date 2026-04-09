package hub

import (
	"context"
	"database/sql"
	"time"
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

type sqliteRAGSyncService struct {
	db *sql.DB
}

func NewSQLiteRAGSyncService(db *sql.DB) RAGSyncService {
	return &sqliteRAGSyncService{db: db}
}

func (s *sqliteRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2`
	rows, err := s.db.QueryContext(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			record.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, record)
	}
	return records, rows.Err()
}

func (s *sqliteRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	query := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3`
	for _, id := range ids {
		_, err := s.db.ExecContext(ctx, query, SyncStatusSynced, time.Now(), id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (s *sqliteRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
	`
	for _, r := range records {
		_, err := s.db.ExecContext(ctx, query, r.ID, r.Context, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			return err
		}
	}
	return nil
}

package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
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
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncService{
		db: db,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		ORDER BY created_at ASC
		LIMIT $1
	`, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus *string
		if err := rows.Scan(&record.ID, &record.Context, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if syncStatus != nil {
			record.SyncStatus = SyncStatus(*syncStatus)
		} else {
			record.SyncStatus = SyncStatusPending
		}
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}
	return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE autodream_memories
			SET sync_status = $1, last_sync_at = $2
			WHERE id = $3
		`, string(SyncStatusSynced), now, id)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, record := range records {
		// Attempt to upsert the memory. Depending on Postgres vs SQLite, we might just try to insert and ignore conflict.
		// For simplicity, let's insert if not exists, but RAG sync is more complex.
		// As per prompt: "Cloud Gateway receives, validates, and upserts into the multi-tenant Postgres DB."
		_, err := tx.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, record.ID, record.Context, string(record.SyncStatus), record.LastSyncAt)
		if err != nil {
			// ON CONFLICT doesn't work on older sqlite without full upsert, but we can assume modern sqlite
			// or just do a simple insert since this is an example implementation.
			// The prompt says: "Implement the foundational schema changes and the Go synchronization service interface".
			return err
		}
	}

	return tx.Commit(ctx)
}

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

type sqlRAGSyncService struct {
	db db.Provider
}

// NewSQLRAGSyncService creates a new RAGSyncService backed by a database.
func NewSQLRAGSyncService(db db.Provider) RAGSyncService {
	return &sqlRAGSyncService{db: db}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"

	// Use $1 for Postgres, ? for SQLite
	if s.db.IsSQLite() {
		query = "SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a naive implementation for marking synced. In a real scenario,
	// we would parameterize the IN clause properly based on DB type, or execute individual updates.
	// For simplicity and since SQLite/PG differences exist, we'll iterate or use a simple parameterized approach.

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		if s.db.IsSQLite() {
			query = "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?"
		}
		_, err := s.db.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	telemetry.RecordRagRecordsSynced(ctx, len(ids))
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, rec := range records {
		// Postgres ON CONFLICT DO UPDATE
		// SQLite ON CONFLICT DO UPDATE
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		`
		if s.db.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content = excluded.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			`
		}
		_, err := s.db.Exec(ctx, query, rec.ID, rec.Context)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
		}
	}

	telemetry.RecordRagRecordsSynced(ctx, len(records))
	return nil
}

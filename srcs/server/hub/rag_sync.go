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
	ID           string
	Context      string
	Vector       []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
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
	dbProvider *db.DB
}

func NewRAGSyncService(dbProvider *db.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: dbProvider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}

	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = $1
			WHERE id = $2
		`
		_, err := s.dbProvider.Exec(ctx, query, time.Now(), id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
		telemetry.RecordRAGRecordSynced(ctx)
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, 'synced', $3)
			ON CONFLICT (id) DO UPDATE
			SET content = EXCLUDED.content,
			    sync_status = 'synced',
			    last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := s.dbProvider.Exec(ctx, query, rec.ID, rec.Context, time.Now())
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
		telemetry.RecordRAGRecordSynced(ctx)
	}
	return nil
}

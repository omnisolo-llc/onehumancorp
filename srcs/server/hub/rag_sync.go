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
	dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &DefaultRAGSyncService{dbProvider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.dbProvider.Query(ctx, "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		if err := rows.Scan(&r.ID, &r.Context, &syncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return nil, err
		}
		r.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		_, err := s.dbProvider.Exec(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2", string(SyncStatusSynced), id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return err
		}
	}

	telemetry.RecordRagRecordsSynced(ctx, len(ids))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		_, err := s.dbProvider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP",
			r.ID, r.Context, string(SyncStatusSynced))
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return err
		}
	}

	telemetry.RecordRagRecordsSynced(ctx, len(records))
	return nil
}

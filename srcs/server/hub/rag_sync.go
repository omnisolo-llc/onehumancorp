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
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Note: In an actual implementation, Vector (embedding) retrieval will require casting or proper formatting based on the driver (pgvector for Postgres, array for sqlite if custom plugin used). For the scope of this implementation, we will select it as a string or leave to be integrated. For now we will only get content, sync_status, last_sync_at since schema isn't natively handling float array to []float32 mapping trivially without proper db mapping layer.
	// As per the code review, we will at least include `embedding` in the select.
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT "
	var q string
	if s.provider.IsSQLite() {
		q = query + "?"
	} else {
		q = query + "$1"
	}

	rows, err := s.provider.Query(ctx, q, limit)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		var embedding interface{}
		if err := rows.Scan(&r.ID, &r.Context, &embedding, &syncStatus, &lastSyncAt); err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		r.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		// If we wanted to map embedding interface{} -> []float32 we would do it here.
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = "
		var q string
		if s.provider.IsSQLite() {
			q = query + "?"
		} else {
			q = query + "$1"
		}
		_, err := s.provider.Exec(ctx, q, id)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}
	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		var q string
		if s.provider.IsSQLite() {
			q = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)
ON CONFLICT(id) DO UPDATE SET content=excluded.content, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
			_, err := s.provider.Exec(ctx, q, r.ID, r.Context)
			if err != nil {
				if telemetry.RagSyncErrorsTotal != nil {
					telemetry.RagSyncErrorsTotal.Add(ctx, 1)
				}
				return err
			}
		} else {
			// For Postgres we could update embedding as well but need a way to format []float32.
			q = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
			_, err := s.provider.Exec(ctx, q, r.ID, r.Context)
			if err != nil {
				if telemetry.RagSyncErrorsTotal != nil {
					telemetry.RagSyncErrorsTotal.Add(ctx, 1)
				}
				return err
			}
		}

	}
	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}

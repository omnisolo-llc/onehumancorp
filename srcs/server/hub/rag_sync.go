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
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var ls *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &ls); err != nil {
			return nil, err
		}
		if ls != nil {
			r.LastSyncAt = *ls
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
		_, err := s.provider.Exec(ctx, query, id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	successCount := 0
	errCount := 0

	for _, r := range records {
		var q string
		var err error
		if s.provider.IsSQLite() {
			// SQLite upsert
			var exists int
			err = s.provider.QueryRow(ctx, "SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = $1", r.ID).Scan(&exists)
			if err == nil {
				q = `UPDATE swarm_memory_embeddings SET context = $2, vector_embedding = $3, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
				_, err = s.provider.Exec(ctx, q, r.ID, r.Context, r.Vector)
			} else {
				q = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)`
				_, err = s.provider.Exec(ctx, q, r.ID, r.Context, r.Vector)
			}
		} else {
			// Postgres upsert
			q = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                 VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
                 ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
			_, err = s.provider.Exec(ctx, q, r.ID, r.Context, r.Vector)
		}

		if err != nil {
			errCount++
		} else {
			successCount++
		}
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(successCount))
	}
	if telemetry.RAGSyncErrorsTotal != nil {
		telemetry.RAGSyncErrorsTotal.Add(ctx, int64(errCount))
	}
	return nil
}

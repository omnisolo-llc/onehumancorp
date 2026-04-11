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

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		if err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt); err != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}
		r.Vector = vector
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = ANY($1)
	`
	if s.provider.IsSQLite() {
		for _, id := range ids {
			q := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
			_, err := s.provider.Exec(ctx, q, id)
			if err != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
				return err
			}
			telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
		}
		return nil
	}

	_, err := s.provider.Exec(ctx, query, ids)
	if err != nil {
		telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	for _, r := range records {
		var query string
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		} else {
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = EXCLUDED.context,
					vector_embedding = EXCLUDED.vector_embedding,
					sync_status = EXCLUDED.sync_status,
					last_sync_at = EXCLUDED.last_sync_at
			`
		}
		_, err := s.provider.Exec(ctx, query, r.ID, r.Context, r.Vector, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
		telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
	}
	return nil
}

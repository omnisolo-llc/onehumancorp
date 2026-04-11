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

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
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
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, id := range ids {
        _, err = tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
        if telemetry.RagRecordsSyncedTotal != nil {
            telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
        }
    }
    return tx.Commit(ctx)
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.provider.IsSQLite() {
			query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = excluded.last_sync_at
			`
		}

		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, now)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		if telemetry.RagRecordsSyncedTotal != nil {
			telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return tx.Commit(ctx)
}

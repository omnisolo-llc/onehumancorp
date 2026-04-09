package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DBProvider interface {
	Exec(ctx context.Context, query string, args ...any) (int64, error)
	Query(ctx context.Context, query string, args ...any) (db.Rows, error)
	IsSQLite() bool
}

type DefaultRAGSyncService struct {
	db DBProvider
}

func NewDefaultRAGSyncService(db DBProvider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.db.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorBytes []byte
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("scan pending sync: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		if len(vectorBytes) > 0 {
			if s.db.IsSQLite() {
				if err := json.Unmarshal(vectorBytes, &r.Vector); err != nil {
					return nil, fmt.Errorf("unmarshal vector (sqlite): %w", err)
				}
			} else {
				// PostgreSQL vector parsing
			}
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = $1, last_sync_at = $2
			WHERE memory_id = $3
		`
		_, err := s.db.Exec(ctx, query, SyncStatusSynced, time.Now(), id)
		if err != nil {
			if RagSyncErrorsTotal != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("mark synced id %s: %w", id, err)
		}
		if RagRecordsSyncedTotal != nil {
			RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		var vectorBytes []byte
		if s.db.IsSQLite() {
			if len(r.Vector) > 0 {
				b, err := json.Marshal(r.Vector)
				if err != nil {
					return fmt.Errorf("marshal vector for id %s: %w", r.ID, err)
				}
				vectorBytes = b
			}
		} else {
			// pgvector conversion logic here
		}

		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := s.db.Exec(ctx, query, r.ID, r.Context, vectorBytes, SyncStatusSynced, time.Now())
		if err != nil {
			if RagSyncErrorsTotal != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("process incoming sync for id %s: %w", r.ID, err)
		}
		if RagRecordsSyncedTotal != nil {
			RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return nil
}

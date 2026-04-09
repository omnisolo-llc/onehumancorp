package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type HybridRAGSyncService struct {
	provider db.Provider
}

func NewHybridRAGSyncService(provider db.Provider) *HybridRAGSyncService {
	return &HybridRAGSyncService{
		provider: provider,
	}
}

func (s *HybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// We only fetch pending syncs when we are standalone
	if !s.provider.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs is only supported in Standalone Mode")
	}

	rows, err := s.provider.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var id, ctxStr, status string
		var vectorJSON []byte
		if err := rows.Scan(&id, &ctxStr, &vectorJSON, &status); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		var vec []float32
		if len(vectorJSON) > 0 {
			if err := json.Unmarshal(vectorJSON, &vec); err != nil {
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
		}

		records = append(records, RAGSyncRecord{
			ID:         id,
			Context:    ctxStr,
			Vector:     vec,
			SyncStatus: SyncStatus(status),
		})
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *HybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	if !s.provider.IsSQLite() {
		return fmt.Errorf("MarkSynced is only supported in Standalone Mode")
	}

	// simple transaction for SQLite
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		// since we just have slice, we can iterate
		_, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
		if err != nil {
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

func (s *HybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.provider.IsSQLite() {
		return fmt.Errorf("ProcessIncomingSync is only supported in Cloud Mode")
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// simple upsert to PostgreSQL. assuming conflict on memory_id
		vecJSON, _ := json.Marshal(r.Vector)
		_, err := tx.Exec(ctx, `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at
        `, r.ID, r.Context, vecJSON)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Only add metrics after successful commit
	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}

var (
	meter                    = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	RagSyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
)

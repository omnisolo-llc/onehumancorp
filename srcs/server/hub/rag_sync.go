package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to cloud"),
	)
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

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

type ragSyncServiceImpl struct {
	dbWrapper *db.DB
}

func NewRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &ragSyncServiceImpl{dbWrapper: dbWrapper}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecData []byte
		rec.SyncStatus = SyncStatusPending

		if err := rows.Scan(&rec.ID, &rec.Context, &vecData); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			continue
		}

		if vecData != nil {
			_ = json.Unmarshal(vecData, &rec.Vector)
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	if s.dbWrapper.IsSQLite() {
		// SQLite might not support $1 = ANY array easily without modernc/sqlite array support,
		// but since it's local, N queries is acceptable for small batches.
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
		for _, id := range ids {
			if _, err := tx.Exec(ctx, query, id); err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to update sync status for %s: %w", id, err)
			}
		}
	} else {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ANY($1)"
		if _, err := tx.Exec(ctx, query, ids); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update sync status: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP"

	for _, rec := range records {
		var vecData []byte
		if rec.Vector != nil {
			var err error
			if vecData, err = json.Marshal(rec.Vector); err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
		}
		if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecData); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}

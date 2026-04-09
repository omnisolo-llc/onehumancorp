package hub

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	_ "go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter               = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedMetric metric.Int64Counter
	syncErrorsMetric    metric.Int64Counter
)

func init() {
	var err error
	recordsSyncedMetric, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	if err != nil {
		panic(err)
	}
	syncErrorsMetric, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total errors during RAG sync"))
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

type sqlRAGSyncService struct {
	database *db.DB
}

func NewSQLRAGSyncService(database *db.DB) RAGSyncService {
	return &sqlRAGSyncService{database: database}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.database.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecJSON []byte
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &vecJSON, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if vecJSON != nil {
			_ = json.Unmarshal(vecJSON, &r.Vector)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		_, err := s.database.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3", string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			syncErrorsMetric.Add(ctx, 1)
			return err
		}
	}

	recordsSyncedMetric.Add(ctx, int64(len(ids)))
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		vecJSON, _ := json.Marshal(r.Vector)

		_, err := s.database.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, r.ID, r.Context, vecJSON, string(r.SyncStatus), r.LastSyncAt)
		if err != nil {
			syncErrorsMetric.Add(ctx, 1)
			return err
		}
	}

	recordsSyncedMetric.Add(ctx, int64(len(records)))
	return nil
}

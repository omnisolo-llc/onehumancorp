package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSynced, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	SyncErrors, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
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

type DefaultRAGSyncService struct {
	Provider db.Provider
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.Provider.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync *time.Time
		var syncStatusStr *string
		var vectorBytes []byte
		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &syncStatusStr, &lastSync); err != nil {
			SyncErrors.Add(ctx, 1)
			return nil, err
		}
		if vectorBytes != nil {
			var vec []float32
			if err := json.Unmarshal(vectorBytes, &vec); err == nil {
				r.Vector = vec
			}
		}
		if syncStatusStr != nil {
			r.SyncStatus = SyncStatus(*syncStatusStr)
		} else {
			r.SyncStatus = SyncStatusPending
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		records = append(records, r)
	}
	if rows.Err() != nil {
		SyncErrors.Add(ctx, 1)
		return nil, rows.Err()
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	args := make([]interface{}, len(ids))
	placeholders := make([]string, len(ids))
	for i, id := range ids {
		args[i] = id
		placeholders[i] = fmt.Sprintf("$%d", i+1)
	}
	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE memory_id IN (%s)", strings.Join(placeholders, ","))

	_, err := s.Provider.Exec(ctx, query, args...)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return err
	}
	RecordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		var vectorBytes []byte
		if r.Vector != nil {
			vectorBytes, _ = json.Marshal(r.Vector)
		}
		_, err := s.Provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (memory_id) DO UPDATE SET context = $2, vector_embedding = $3, sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP", r.ID, r.Context, vectorBytes)
		if err != nil {
			SyncErrors.Add(ctx, 1)
			return err
		}
		RecordsSynced.Add(ctx, 1)
	}
	return nil
}

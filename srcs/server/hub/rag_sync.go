package hub

import (
	"context"
	"time"
	"log/slog"
	"fmt"
	"strings"
	"github.com/onehumancorp/mono/srcs/server/db"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         metric.Meter
	RecordsSynced metric.Int64Counter
	SyncErrors    metric.Int64Counter
)

func init() {
	var err error
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		slog.Error("Failed to initialize RecordsSynced metric", "error", err)
	}
	SyncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		slog.Error("Failed to initialize SyncErrors metric", "error", err)
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

type HybridRAGSyncService struct {
	dbProvider db.Provider
}

func NewHybridRAGSyncService(dbProvider db.Provider) *HybridRAGSyncService {
	return &HybridRAGSyncService{dbProvider: dbProvider}
}

func (s *HybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.dbProvider.Query(ctx, "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan pending syncs: %w", err)
		}
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}
	return records, nil
}

func (s *HybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)", strings.Join(placeholders, ","))

	_, err := s.dbProvider.Exec(ctx, query, args...)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	RecordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *HybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, record := range records {
		_, err := s.dbProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP", record.ID, record.Context)
		if err != nil {
			SyncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync: %w", err)
		}
	}
	RecordsSynced.Add(ctx, int64(len(records)))
	return nil
}

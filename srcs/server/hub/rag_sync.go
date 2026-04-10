package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
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
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		db: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT id, content, embedding, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var id, contextData, syncStatus string
		var lastSyncAt sql.NullTime
		var embeddingStr *string

		if err := rows.Scan(&id, &contextData, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan pending syncs: %w", err)
		}

		record := RAGSyncRecord{
			ID:         id,
			Context:    contextData,
			SyncStatus: SyncStatus(syncStatus),
		}
		if lastSyncAt.Valid {
			record.LastSyncAt = lastSyncAt.Time
		}

		if embeddingStr != nil && *embeddingStr != "" {
			// Embedding can be stored as a stringified JSON array or vector literal "[1,2,3]"
			cleanStr := strings.Trim(strings.TrimSpace(*embeddingStr), "[]")
			if cleanStr != "" {
				var vec []float32
				err := json.Unmarshal([]byte("["+cleanStr+"]"), &vec)
				if err == nil {
					record.Vector = vec
				}
			}
		}
		records = append(records, record)
	}
	return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		_, err := s.db.Exec(ctx, "UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		var vecStr *string
		if len(r.Vector) > 0 {
			b, _ := json.Marshal(r.Vector)
			s := string(b)
			vecStr = &s
		}

		_, err := s.db.Exec(ctx, `
			INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
			VALUES ($1, 'system', $2, $3, 'hybrid_sync', 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`, r.ID, r.Context, vecStr)
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func InitMetrics(meter metric.Meter) error {
	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced to cloud"))
	if err != nil {
		return err
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total errors during RAG sync"))
	if err != nil {
		return err
	}

	return nil
}
